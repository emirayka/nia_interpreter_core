use crate::{Convertable, Key, ModifierDescription};

#[derive(Clone, Debug, Eq, Hash)]
pub struct KeyChord {
    modifiers: Vec<Key>,
    key: Key,
}

impl KeyChord {
    pub fn from_one_key(key: Key) -> KeyChord {
        KeyChord {
            modifiers: Vec::new(),
            key,
        }
    }

    pub fn from_two_keys(modifier1: Key, key: Key) -> KeyChord {
        KeyChord {
            modifiers: vec![modifier1],
            key,
        }
    }

    pub fn from_three_keys(
        modifier1: Key,
        modifier2: Key,
        key: Key,
    ) -> KeyChord {
        KeyChord {
            modifiers: vec![modifier1, modifier2],
            key,
        }
    }

    pub fn from_four_keys(
        modifier1: Key,
        modifier2: Key,
        modifier3: Key,
        key: Key,
    ) -> KeyChord {
        KeyChord {
            modifiers: vec![modifier1, modifier2, modifier3],
            key,
        }
    }

    pub fn from_five_keys(
        modifier1: Key,
        modifier2: Key,
        modifier3: Key,
        modifier4: Key,
        key: Key,
    ) -> KeyChord {
        KeyChord {
            modifiers: vec![modifier1, modifier2, modifier3, modifier4],
            key,
        }
    }

    pub fn new(modifiers: Vec<Key>, key: Key) -> KeyChord {
        KeyChord { modifiers, key }
    }

    pub fn get_modifiers(&self) -> &Vec<Key> {
        &self.modifiers
    }

    pub fn get_key(&self) -> Key {
        self.key
    }

    pub fn key_chords_are_same(
        key_chord_1: &KeyChord,
        key_chord_2: &KeyChord,
    ) -> bool {
        if !Key::keys_are_same(key_chord_1.get_key(), key_chord_2.get_key()) {
            return false;
        }

        if key_chord_1.get_modifiers().len()
            != key_chord_2.get_modifiers().len()
        {
            return false;
        }

        for modifier_1 in key_chord_1.get_modifiers() {
            let mut key_chord_1_modifier_not_found_in_key_chord_2_modifiers =
                true;

            for modifier_2 in key_chord_2.get_modifiers() {
                if Key::keys_are_same(*modifier_1, *modifier_2) {
                    key_chord_1_modifier_not_found_in_key_chord_2_modifiers =
                        false;
                    break;
                }
            }

            if key_chord_1_modifier_not_found_in_key_chord_2_modifiers {
                return false;
            }
        }

        for modifier_2 in key_chord_2.get_modifiers() {
            let mut key_chord_2_modifier_not_found_in_key_chord_1_modifiers =
                true;

            for modifier_1 in key_chord_1.get_modifiers() {
                if Key::keys_are_same(*modifier_1, *modifier_2) {
                    key_chord_2_modifier_not_found_in_key_chord_1_modifiers =
                        false;
                    break;
                }
            }

            if key_chord_2_modifier_not_found_in_key_chord_1_modifiers {
                return false;
            }
        }

        true
    }

    pub fn key_chord_vectors_are_same(
        key_chord_vector_1: &Vec<KeyChord>,
        key_chord_vector_2: &Vec<KeyChord>,
    ) -> bool {
        if key_chord_vector_1.len() != key_chord_vector_2.len() {
            return false;
        }

        let mut iterator =
            key_chord_vector_1.iter().zip(key_chord_vector_2.iter());

        for (key_chord_1, key_chord_2) in iterator {
            if !KeyChord::key_chords_are_same(key_chord_1, key_chord_2) {
                return false;
            }
        }

        return true;
    }
}

impl PartialEq for KeyChord {
    fn eq(&self, other: &Self) -> bool {
        if self.key != other.key {
            return false;
        }

        if self.modifiers.len() != other.modifiers.len() {
            return false;
        }

        for key in &self.modifiers {
            if !other.modifiers.contains(key) {
                return false;
            }
        }

        for key in &other.modifiers {
            if !self.modifiers.contains(key) {
                return false;
            }
        }

        return true;
    }
}

impl Convertable<KeyChord, nia_events::KeyChord> for KeyChord {
    fn to_nia_events_representation(&self) -> nia_events::KeyChord {
        let modifiers_et = self
            .modifiers
            .iter()
            .map(|modifier| modifier.to_nia_events_representation())
            .collect();

        let ordinary_key_et = self.key.to_nia_events_representation();

        nia_events::KeyChord::new(modifiers_et, ordinary_key_et)
    }

    fn from_nia_events_representation(
        value: &nia_events::KeyChord,
    ) -> KeyChord {
        let modifiers = value
            .get_modifiers()
            .iter()
            .map(|modifier_key_et| {
                Key::from_nia_events_representation(modifier_key_et)
            })
            .collect();

        let ordinary_key = Key::from_nia_events_representation(value.get_key());

        KeyChord::new(modifiers, ordinary_key)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use nia_basic_assertions::nia_assert_equal;

    #[cfg(test)]
    mod eq {
        #[allow(unused_imports)]
        use super::*;

        fn assert_equality_result_is_correct(
            specs: Vec<(KeyChord, KeyChord, bool)>,
        ) {
            for (key_chord_1, key_chord_2, expected) in specs {
                let result = key_chord_1 == key_chord_2;

                nia_assert_equal(expected, result);
            }
        }

        #[test]
        fn returns_true_when_keys_and_modifiers_are_equal() {
            let specs = vec![
                (
                    KeyChord::new(vec![], nia_key!(1)),
                    KeyChord::new(vec![], nia_key!(1)),
                    true,
                ),
                (
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1)),
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1)),
                    true,
                ),
                (
                    KeyChord::new(vec![nia_key!(3), nia_key!(4)], nia_key!(2)),
                    KeyChord::new(vec![nia_key!(4), nia_key!(3)], nia_key!(2)),
                    true,
                ),
            ];

            assert_equality_result_is_correct(specs)
        }

        #[test]
        fn returns_false_if_modifiers_are_not_equal() {
            let specs = vec![
                (
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1)),
                    KeyChord::new(vec![], nia_key!(1)),
                    false,
                ),
                (
                    KeyChord::new(vec![], nia_key!(1)),
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1)),
                    false,
                ),
                (
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1)),
                    KeyChord::new(vec![nia_key!(4)], nia_key!(1)),
                    false,
                ),
                (
                    KeyChord::new(vec![nia_key!(4)], nia_key!(1)),
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1)),
                    false,
                ),
                (
                    KeyChord::new(vec![nia_key!(4)], nia_key!(2)),
                    KeyChord::new(vec![nia_key!(4), nia_key!(3)], nia_key!(2)),
                    false,
                ),
                (
                    KeyChord::new(vec![nia_key!(3), nia_key!(4)], nia_key!(2)),
                    KeyChord::new(vec![nia_key!(3)], nia_key!(2)),
                    false,
                ),
                (
                    KeyChord::new(vec![nia_key!(3), nia_key!(4)], nia_key!(2)),
                    KeyChord::new(vec![nia_key!(5), nia_key!(4)], nia_key!(2)),
                    false,
                ),
            ];

            assert_equality_result_is_correct(specs)
        }

        #[test]
        fn returns_false_if_keys_are_not_equal() {
            let specs = vec![
                (
                    KeyChord::new(vec![], nia_key!(1)),
                    KeyChord::new(vec![], nia_key!(2)),
                    false,
                ),
                (
                    KeyChord::new(vec![], nia_key!(2)),
                    KeyChord::new(vec![], nia_key!(1)),
                    false,
                ),
                (
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1)),
                    KeyChord::new(vec![nia_key!(3)], nia_key!(2)),
                    false,
                ),
                (
                    KeyChord::new(vec![nia_key!(3)], nia_key!(2)),
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1)),
                    false,
                ),
            ];

            assert_equality_result_is_correct(specs)
        }

        #[test]
        fn returns_correct_equality_results() {
            let specs = vec![
                (
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1, 1)),
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1)),
                    true,
                ),
                (
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1)),
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1, 1)),
                    true,
                ),
                (
                    KeyChord::new(vec![nia_key!(1, 3)], nia_key!(1)),
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1)),
                    true,
                ),
                (
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1)),
                    KeyChord::new(vec![nia_key!(1, 3)], nia_key!(1)),
                    true,
                ),
            ];

            assert_equality_result_is_correct(specs)
        }
    }

    #[cfg(test)]
    mod key_chords_are_same {
        #[allow(unused_imports)]
        use super::*;

        fn assert_same_results_are_correct(
            specs: Vec<(KeyChord, KeyChord, bool)>,
        ) {
            for (key_chord_1, key_chord_2, expected) in specs {
                let result =
                    KeyChord::key_chords_are_same(&key_chord_1, &key_chord_2);

                nia_assert_equal(expected, result);
            }
        }

        #[test]
        fn returns_true_when_keys_and_modifiers_are_same() {
            let specs = vec![
                (
                    KeyChord::new(vec![], nia_key!(1)),
                    KeyChord::new(vec![], nia_key!(1)),
                    true,
                ),
                (
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1)),
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1)),
                    true,
                ),
                (
                    KeyChord::new(vec![nia_key!(3), nia_key!(4)], nia_key!(2)),
                    KeyChord::new(vec![nia_key!(4), nia_key!(3)], nia_key!(2)),
                    true,
                ),
            ];

            assert_same_results_are_correct(specs)
        }

        #[test]
        fn returns_false_if_modifiers_are_not_equal() {
            let specs = vec![
                (
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1)),
                    KeyChord::new(vec![], nia_key!(1)),
                    false,
                ),
                (
                    KeyChord::new(vec![], nia_key!(1)),
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1)),
                    false,
                ),
                (
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1)),
                    KeyChord::new(vec![nia_key!(4)], nia_key!(1)),
                    false,
                ),
                (
                    KeyChord::new(vec![nia_key!(4)], nia_key!(1)),
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1)),
                    false,
                ),
                (
                    KeyChord::new(vec![nia_key!(4)], nia_key!(2)),
                    KeyChord::new(vec![nia_key!(4), nia_key!(3)], nia_key!(2)),
                    false,
                ),
                (
                    KeyChord::new(vec![nia_key!(3), nia_key!(4)], nia_key!(2)),
                    KeyChord::new(vec![nia_key!(3)], nia_key!(2)),
                    false,
                ),
                (
                    KeyChord::new(vec![nia_key!(3), nia_key!(4)], nia_key!(2)),
                    KeyChord::new(vec![nia_key!(5), nia_key!(4)], nia_key!(2)),
                    false,
                ),
            ];

            assert_same_results_are_correct(specs)
        }

        #[test]
        fn returns_false_if_keys_are_not_equal() {
            let specs = vec![
                (
                    KeyChord::new(vec![], nia_key!(1)),
                    KeyChord::new(vec![], nia_key!(2)),
                    false,
                ),
                (
                    KeyChord::new(vec![], nia_key!(2)),
                    KeyChord::new(vec![], nia_key!(1)),
                    false,
                ),
                (
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1)),
                    KeyChord::new(vec![nia_key!(3)], nia_key!(2)),
                    false,
                ),
                (
                    KeyChord::new(vec![nia_key!(3)], nia_key!(2)),
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1)),
                    false,
                ),
            ];

            assert_same_results_are_correct(specs)
        }

        #[test]
        fn returns_correct_same_results() {
            let specs = vec![
                (
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1, 1)),
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1)),
                    false,
                ),
                (
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1)),
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1, 1)),
                    false,
                ),
                (
                    KeyChord::new(vec![nia_key!(1, 3)], nia_key!(1)),
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1)),
                    false,
                ),
                (
                    KeyChord::new(vec![nia_key!(3)], nia_key!(1)),
                    KeyChord::new(vec![nia_key!(1, 3)], nia_key!(1)),
                    false,
                ),
            ];

            assert_same_results_are_correct(specs)
        }
    }
}
