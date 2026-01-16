// ============================================
// FICHIER: src/main.rs
// ============================================

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
struct LogStats {
    total_lines: usize,
    levels: HashMap<String, usize>,
    errors: Vec<String>,
    warnings: Vec<String>,
    ip_addresses: HashMap<String, usize>,
    status_codes: HashMap<u16, usize>,
}

impl LogStats {
    fn new() -> Self {
        LogStats {
            total_lines: 0,
            levels: HashMap::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
            ip_addresses: HashMap::new(),
            status_codes: HashMap::new(),
        }
    }

    fn analyze_line(&mut self, line: &str) {
        self.total_lines += 1;

        // Détecter les niveaux de log (INFO, ERROR, WARN, DEBUG)
        let line_upper = line.to_uppercase();
        for level in ["ERROR", "WARN", "WARNING", "INFO", "DEBUG"] {
            if line_upper.contains(level) {
                *self.levels.entry(level.to_string()).or_insert(0) += 1;
                
                if level == "ERROR" {
                    self.errors.push(line.to_string());
                } else if level == "WARN" || level == "WARNING" {
                    self.warnings.push(line.to_string());
                }
                break;
            }
        }

        // Extraire les adresses IP (format simple)
        if let Some(ip) = extract_ip(line) {
            *self.ip_addresses.entry(ip).or_insert(0) += 1;
        }

        // Extraire les codes de statut HTTP
        if let Some(code) = extract_status_code(line) {
            *self.status_codes.entry(code).or_insert(0) += 1;
        }
    }

    fn print_report(&self) {
        println!("\n╔══════════════════════════════════════╗");
        println!("║   RAPPORT D'ANALYSE DES LOGS         ║");
        println!("╚══════════════════════════════════════╝\n");

        println!("📊 Total de lignes analysées : {}\n", self.total_lines);

        // Niveaux de log
        if !self.levels.is_empty() {
            println!("📋 Répartition par niveau :");
            let mut levels: Vec<_> = self.levels.iter().collect();
            levels.sort_by(|a, b| b.1.cmp(a.1));
            for (level, count) in levels {
                let percentage = (*count as f64 / self.total_lines as f64) * 100.0;
                println!("   {} : {} ({:.1}%)", level, count, percentage);
            }
            println!();
        }

        // Codes de statut HTTP
        if !self.status_codes.is_empty() {
            println!("🌐 Codes de statut HTTP :");
            let mut codes: Vec<_> = self.status_codes.iter().collect();
            codes.sort_by_key(|&(code, _)| code);
            for (code, count) in codes {
                println!("   {} : {}", code, count);
            }
            println!();
        }

        // Top 10 des adresses IP
        if !self.ip_addresses.is_empty() {
            println!("🔝 Top 10 des adresses IP :");
            let mut ips: Vec<_> = self.ip_addresses.iter().collect();
            ips.sort_by(|a, b| b.1.cmp(a.1));
            for (ip, count) in ips.iter().take(10) {
                println!("   {} : {} requêtes", ip, count);
            }
            println!();
        }

        // Erreurs récentes
        if !self.errors.is_empty() {
            println!("❌ Dernières erreurs ({}) :", self.errors.len());
            for error in self.errors.iter().rev().take(5) {
                println!("   {}", error.trim());
            }
            println!();
        }

        // Avertissements récents
        if !self.warnings.is_empty() {
            println!("⚠️  Derniers avertissements ({}) :", self.warnings.len());
            for warning in self.warnings.iter().rev().take(5) {
                println!("   {}", warning.trim());
            }
            println!();
        }
    }
}

fn extract_ip(line: &str) -> Option<String> {
    // Regex simple pour IPv4
    for word in line.split_whitespace() {
        let parts: Vec<&str> = word.split('.').collect();
        if parts.len() == 4 && parts.iter().all(|p| p.parse::<u8>().is_ok()) {
            return Some(word.to_string());
        }
    }
    None
}

fn extract_status_code(line: &str) -> Option<u16> {
    // Cherche des nombres à 3 chiffres commençant par 1-5 (codes HTTP)
    for word in line.split_whitespace() {
        if let Ok(code) = word.parse::<u16>() {
            if (100..600).contains(&code) {
                return Some(code);
            }
        }
    }
    None
}

fn analyze_log_file<P: AsRef<Path>>(path: P) -> Result<LogStats, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut stats = LogStats::new();

    for line in reader.lines() {
        let line = line?;
        stats.analyze_line(&line);
    }

    Ok(stats)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <fichier_log>", args[0]);
        eprintln!("\nExemple: {} /var/log/nginx/access.log", args[0]);
        std::process::exit(1);
    }

    let log_file = &args[1];
    
    println!("🔍 Analyse du fichier : {}", log_file);
    
    match analyze_log_file(log_file) {
        Ok(stats) => stats.print_report(),
        Err(e) => {
            eprintln!("❌ Erreur lors de la lecture du fichier : {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_ip() {
        assert_eq!(extract_ip("192.168.1.1 - request"), Some("192.168.1.1".to_string()));
        assert_eq!(extract_ip("no ip here"), None);
    }

    #[test]
    fn test_extract_status_code() {
        assert_eq!(extract_status_code("GET /page 200 OK"), Some(200));
        assert_eq!(extract_status_code("Error 404 not found"), Some(404));
        assert_eq!(extract_status_code("no status code"), None);
    }
}