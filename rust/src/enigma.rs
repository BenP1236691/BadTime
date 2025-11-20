pub struct EnigmaMachine {
    rotors: [Rotor; 3],
    reflector: Reflector,
    plugboard: Vec<(char, char)>,
}

#[derive(Clone)]
struct Rotor {
    wiring: Vec<char>,
    notch: char,
    position: usize,
    ring_setting: usize,
}

struct Reflector {
    wiring: Vec<char>,
}

impl EnigmaMachine {
    pub fn new(rotors: [(usize, char, char); 3], reflector_type: char, plugboard_pairs: &str) -> Self {
        let rotor_configs = [
            ("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'Q'), // I
            ("AJDKSIRUXBLHWTMCQGZNPYFVOE", 'E'), // II
            ("BDFHJLCPRTXVZNYEIWGAKMUSQO", 'V'), // III
        ];

        let created_rotors = rotors.map(|(idx, ring, start)| {
            let (wiring_str, notch) = rotor_configs[idx]; // 0-indexed rotor choice
            Rotor {
                wiring: wiring_str.chars().collect(),
                notch,
                position: (start as u8 - b'A') as usize,
                ring_setting: (ring as u8 - b'A') as usize,
            }
        });

        let reflector_wiring = match reflector_type {
            'B' => "YRUHQSLDPXNGOKMIEBFZCWVJAT",
            _ => "YRUHQSLDPXNGOKMIEBFZCWVJAT", // Default to B
        };

        let mut pb = Vec::new();
        for pair in plugboard_pairs.split_whitespace() {
            let chars: Vec<char> = pair.chars().collect();
            if chars.len() == 2 {
                pb.push((chars[0], chars[1]));
            }
        }

        EnigmaMachine {
            rotors: created_rotors,
            reflector: Reflector { wiring: reflector_wiring.chars().collect() },
            plugboard: pb,
        }
    }

    fn map_char(c: char, map: &[(char, char)]) -> char {
        for &(a, b) in map {
            if c == a { return b; }
            if c == b { return a; }
        }
        c
    }

    fn rotor_forward(c: char, rotor: &Rotor) -> char {
        let offset = (rotor.position + 26 - rotor.ring_setting) % 26;
        let idx = (c as u8 - b'A') as usize;
        let input_idx = (idx + offset) % 26;
        let mapped_char = rotor.wiring[input_idx];
        let output_idx = (mapped_char as u8 - b'A' as u8) as usize;
        let final_idx = (output_idx + 26 - offset) % 26;
        (final_idx as u8 + b'A') as char
    }

    fn rotor_backward(c: char, rotor: &Rotor) -> char {
        let offset = (rotor.position + 26 - rotor.ring_setting) % 26;
        let idx = (c as u8 - b'A') as usize;
        let input_idx = (idx + offset) % 26;
        
        // Find which input maps to this output in the wiring
        let mapped_char = (input_idx as u8 + b'A') as char;
        let wiring_idx = rotor.wiring.iter().position(|&x| x == mapped_char).unwrap();
        
        let final_idx = (wiring_idx + 26 - offset) % 26;
        (final_idx as u8 + b'A') as char
    }

    fn step_rotors(&mut self) {
        let r1_at_notch = self.rotors[2].position == (self.rotors[2].notch as u8 - b'A') as usize;
        let r2_at_notch = self.rotors[1].position == (self.rotors[1].notch as u8 - b'A') as usize;

        // Rotor 3 (rightmost) always steps
        let step_r3 = true;
        // Rotor 2 steps if R3 is at notch OR if R2 is at notch (double stepping)
        let step_r2 = r1_at_notch || r2_at_notch;
        // Rotor 1 steps if R2 is at notch
        let step_r1 = r2_at_notch;

        if step_r1 { self.rotors[0].position = (self.rotors[0].position + 1) % 26; }
        if step_r2 { self.rotors[1].position = (self.rotors[1].position + 1) % 26; }
        if step_r3 { self.rotors[2].position = (self.rotors[2].position + 1) % 26; }
    }

    pub fn process_char(&mut self, c: char) -> char {
        if !c.is_ascii_alphabetic() {
            return c;
        }
        let upper = c.to_ascii_uppercase();
        
        self.step_rotors();

        let mut res = Self::map_char(upper, &self.plugboard);
        
        // Forward through rotors (Right to Left: 2 -> 1 -> 0)
        res = Self::rotor_forward(res, &self.rotors[2]);
        res = Self::rotor_forward(res, &self.rotors[1]);
        res = Self::rotor_forward(res, &self.rotors[0]);

        // Reflector
        let idx = (res as u8 - b'A') as usize;
        res = self.reflector.wiring[idx];

        // Backward through rotors (Left to Right: 0 -> 1 -> 2)
        res = Self::rotor_backward(res, &self.rotors[0]);
        res = Self::rotor_backward(res, &self.rotors[1]);
        res = Self::rotor_backward(res, &self.rotors[2]);

        res = Self::map_char(res, &self.plugboard);
        
        // Preserve original case? Enigma is case-insensitive (outputs uppercase).
        // But for code, we might need to handle case.
        // Standard Enigma only does A-Z.
        // For a "Code Runner", we need to support all characters.
        // Strategy: Only encrypt A-Z, leave others alone? 
        // Or map full ASCII?
        // The user said "Enigma", which implies A-Z. 
        // But JS code has symbols. 
        // Let's stick to A-Z encryption for the "Enigma" vibe, and leave symbols as is.
        // This is a "wrapper" after all.
        
        if c.is_ascii_lowercase() {
            res.to_ascii_lowercase()
        } else {
            res
        }
    }

    pub fn process_text(&mut self, text: &str) -> String {
        text.chars().map(|c| self.process_char(c)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symmetry() {
        let mut enigma_enc = EnigmaMachine::new([(0, 'A', 'A'), (1, 'A', 'A'), (2, 'A', 'A')], 'B', "");
        let mut enigma_dec = EnigmaMachine::new([(0, 'A', 'A'), (1, 'A', 'A'), (2, 'A', 'A')], 'B', "");

        let input = "HELLOWORLD";
        let encrypted = enigma_enc.process_text(input);
        let decrypted = enigma_dec.process_text(&encrypted);

        assert_eq!(input, decrypted);
        assert_ne!(input, encrypted);
    }


}
