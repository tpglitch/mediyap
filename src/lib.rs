//! # MediYap
//!
//! A Rust crate for decoding medical terminology into plain English.
//!
//! This crate breaks down medical terms by identifying common prefixes, suffixes,
//! and root words, then translates them into human-readable descriptions.
//!
//! ## Examples
//!
//! ```
//! use mediyap::MedicalDecoder;
//!
//! let decoder = MedicalDecoder::new();
//!
//! assert_eq!(
//!     decoder.decode("hypoglycemia"),
//!     "low glucose/sugar presence in blood"
//! );
//!
//! assert_eq!(
//!     decoder.decode("tachycardia"),
//!     "fast heart"
//! );
//!
//! assert_eq!(
//!     decoder.decode("nephritis"),
//!     "kidney inflammation"
//! );
//! ```
//!
//! ## Medical Term Components
//!
//! - **Prefixes**: hypo- (low), hyper- (high), brady- (slow), tachy- (fast)
//! - **Suffixes**: -emia (presence in blood), -itis (inflammation), -penia (deficiency)
//! - **Roots**: glyc (glucose), card (heart), nephr (kidney), hem (blood)
use std::collections::HashMap;

pub struct MedicalDecoder {
    prefixes: HashMap<&'static str, &'static str>,
    suffixes: HashMap<&'static str, &'static str>,
    roots: HashMap<&'static str, &'static str>,
}

impl MedicalDecoder {
    pub fn new() -> Self {
        let mut prefixes = HashMap::new();
        prefixes.insert("hypo", "low");
        prefixes.insert("hyper", "high");
        prefixes.insert("a", "without");
        prefixes.insert("an", "without");
        prefixes.insert("brady", "slow");
        prefixes.insert("tachy", "fast");
        prefixes.insert("poly", "many");
        prefixes.insert("oligo", "few");
        prefixes.insert("macro", "large");
        prefixes.insert("micro", "small");
        prefixes.insert("eu", "normal");
        prefixes.insert("dys", "difficult/painful/abnormal");
        prefixes.insert("hyster", "uterus");
        prefixes.insert("peri", "around");
        prefixes.insert("endo", "within");
        prefixes.insert("epi", "upon/above");
        prefixes.insert("intra", "within");
        prefixes.insert("inter", "between");
        prefixes.insert("sub", "below");
        prefixes.insert("trans", "across");

        let mut suffixes = HashMap::new();
        suffixes.insert("emia", "presence in blood");
        suffixes.insert("uria", "presence in urine");
        suffixes.insert("itis", "inflammation");
        suffixes.insert("osis", "condition/disease");
        suffixes.insert("pathy", "disease");
        suffixes.insert("penia", "deficiency");
        suffixes.insert("cytosis", "increase in cells");
        suffixes.insert("lysis", "breakdown/destruction");
        suffixes.insert("algia", "pain");
        suffixes.insert("ectomy", "surgical removal");
        suffixes.insert("otomy", "surgical incision");
        suffixes.insert("ostomy", "surgical opening");
        suffixes.insert("plasty", "surgical repair");
        suffixes.insert("scopy", "visual examination");
        suffixes.insert("graphy", "recording process");
        suffixes.insert("gram", "record/image");
        suffixes.insert("megaly", "enlargement");
        suffixes.insert("rrhea", "flow/discharge");
        suffixes.insert("rrhage", "excessive bleeding");
        suffixes.insert("rrhagia", "excessive bleeding");

        let mut roots = HashMap::new();
        roots.insert("glyc", "glucose/sugar");
        roots.insert("gluc", "glucose/sugar");
        roots.insert("card", "heart");
        roots.insert("cardi", "heart");
        roots.insert("hem", "blood");
        roots.insert("hemat", "blood");
        roots.insert("thromb", "clot");
        roots.insert("leuk", "white");
        roots.insert("erythr", "red");
        roots.insert("cyan", "blue");
        roots.insert("neur", "nerve");
        roots.insert("gastr", "stomach");
        roots.insert("hepat", "liver");
        roots.insert("nephr", "kidney");
        roots.insert("ren", "kidney");
        roots.insert("pulmon", "lung");
        roots.insert("pneum", "lung/air");
        roots.insert("dermat", "skin");
        roots.insert("derm", "skin");
        roots.insert("oste", "bone");
        roots.insert("arthr", "joint");
        roots.insert("my", "muscle");
        roots.insert("cyt", "cell");
        roots.insert("path", "disease");
        roots.insert("therm", "heat/temperature");
        roots.insert("py", "pus");
        roots.insert("rhin", "nose");
        roots.insert("ot", "ear");
        roots.insert("ophthalm", "eye");
        roots.insert("angi", "vessel");
        roots.insert("vas", "vessel");
        roots.insert("phleb", "vein");
        roots.insert("arteri", "artery");
        roots.insert("lymph", "lymph");
        roots.insert("immun", "immune");
        roots.insert("toxic", "poison");
        roots.insert("psych", "mind");
        roots.insert("encephal", "brain");
        roots.insert("cerebr", "brain");
        roots.insert("mening", "membrane/meninges");
        roots.insert("vertebr", "spine");
        roots.insert("cost", "rib");
        roots.insert("thorac", "chest");
        roots.insert("abdomin", "abdomen");
        roots.insert("enter", "intestine");
        roots.insert("col", "colon");
        roots.insert("proct", "rectum");
        roots.insert("gastr", "stomach");
        roots.insert("esophag", "esophagus");
        roots.insert("gloss", "tongue");
        roots.insert("dent", "tooth");
        roots.insert("gingiv", "gum");

        MedicalDecoder {
            prefixes,
            suffixes,
            roots,
        }
    }

    pub fn decode(&self, term: &str) -> String {
        let term_lower = term.to_lowercase();
        let mut parts = Vec::new();

        // Find prefix
        let mut remaining = term_lower.as_str();
        for (prefix, meaning) in &self.prefixes {
            if remaining.starts_with(prefix) {
                parts.push(meaning.to_string());
                remaining = &remaining[prefix.len()..];
                break;
            }
        }

        // Find suffix
        let mut suffix_found = None;
        for (suffix, meaning) in &self.suffixes {
            if remaining.ends_with(suffix) {
                suffix_found = Some((suffix.len(), meaning));
                break;
            }
        }

        // Extract root (middle part)
        let root_part = if let Some((suffix_len, _)) = suffix_found {
            &remaining[..remaining.len() - suffix_len]
        } else {
            remaining
        };

        // Find root meaning
        if !root_part.is_empty() {
            let mut root_found = false;
            for (root, meaning) in &self.roots {
                if root_part.starts_with(root) || root_part.contains(root) {
                    parts.push(meaning.to_string());
                    root_found = true;
                    break;
                }
            }
            if !root_found && !parts.is_empty() {
                // If we found prefix but not root, include the unknown part
                parts.push(format!("[{}]", root_part));
            }
        }

        // Add suffix meaning
        if let Some((_, suffix_meaning)) = suffix_found {
            parts.push(suffix_meaning.to_string());
        }

        if parts.is_empty() {
            format!("Unable to decode '{}'", term)
        } else {
            parts.join(" ")
        }
    }
}

impl Default for MedicalDecoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hypoglycemia() {
        let decoder = MedicalDecoder::new();
        assert_eq!(
            decoder.decode("hypoglycemia"),
            "low glucose/sugar presence in blood"
        );
    }

    #[test]
    fn test_hyperglycemia() {
        let decoder = MedicalDecoder::new();
        assert_eq!(
            decoder.decode("hyperglycemia"),
            "high glucose/sugar presence in blood"
        );
    }

    #[test]
    fn test_carditis() {
        let decoder = MedicalDecoder::new();
        assert_eq!(decoder.decode("carditis"), "heart inflammation");
    }

    #[test]
    fn test_tachycardia() {
        let decoder = MedicalDecoder::new();
        assert_eq!(decoder.decode("tachycardia"), "fast heart");
    }

    #[test]
    fn test_leukemia() {
        let decoder = MedicalDecoder::new();
        assert_eq!(decoder.decode("leukemia"), "white presence in blood");
    }

    #[test]
    fn test_thrombocytopenia() {
        let decoder = MedicalDecoder::new();
        assert_eq!(decoder.decode("thrombocytopenia"), "clot cell deficiency");
    }

    #[test]
    fn test_arthritis() {
        let decoder = MedicalDecoder::new();
        assert_eq!(decoder.decode("arthritis"), "joint inflammation");
    }

    #[test]
    fn test_nephritis() {
        let decoder = MedicalDecoder::new();
        assert_eq!(decoder.decode("nephritis"), "kidney inflammation");
    }
}
