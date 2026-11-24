/*
 * Copyright 2013 ZXing authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// use std::collections::HashMap;
use std::collections::BTreeMap;

/**
 * @author Guenther Grau
 */
#[derive(Clone, Default)]
pub struct BarcodeValue(BTreeMap<u32, u32>);

impl BarcodeValue {
    pub fn new() -> Self {
        Self::default()
    }

    /**
     * Add an occurrence of a value
     */
    pub fn setValue(&mut self, value: u32) {
        self.0
            .entry(value)
            .and_modify(|confidence| *confidence += 1)
            .or_insert(1);
    }

    /**
     * Determines the maximum occurrence of a set value and returns all values which were set with this occurrence.
     * @return an array of int, containing the values with the highest occurrence, or null, if no value was set
     */
    pub fn getValue(&self) -> Vec<u32> {
        let mut maxConfidence = -1_i32;
        let mut result = Vec::new();
        for (key, value) in &self.0 {
            match (*value as i32).cmp(&maxConfidence) {
                std::cmp::Ordering::Greater => {
                    maxConfidence = *value as i32;
                    result.clear();
                    result.push(*key);
                }
                std::cmp::Ordering::Equal => result.push(*key),
                std::cmp::Ordering::Less => {}
            }
        }

        result
    }

    pub fn getConfidence(&self, value: u32) -> u32 {
        *self.0.get(&value).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::BarcodeValue;

    /// Values tied for the highest vote count must be returned in the same
    /// order every time, regardless of HashMap hashing.
    #[test]
    fn get_value_ties_are_deterministic() {
        let tied = [929_u32, 3, 517, 42, 800, 1, 255, 600];

        let build = || {
            let mut bv = BarcodeValue::new();
            for v in tied {
                bv.setValue(v);
                bv.setValue(v); // each value gets 2 votes -> all tied
            }
            bv.setValue(7); // 1 vote -> must not appear in the result
            bv
        };

        let mut expected = tied.to_vec();
        expected.sort_unstable();

        for _ in 0..50 {
            assert_eq!(build().getValue(), expected);
        }
    }

    #[test]
    fn get_value_returns_only_max() {
        let mut bv = BarcodeValue::new();
        bv.setValue(10);
        bv.setValue(10);
        bv.setValue(20);
        assert_eq!(bv.getValue(), vec![10]);
        assert_eq!(bv.getConfidence(10), 2);
        assert_eq!(bv.getConfidence(20), 1);
        assert!(BarcodeValue::new().getValue().is_empty());
    }
}
