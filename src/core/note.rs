/*
 * Copyright (C) 2021 Alexi Wolf
 *
 * Fluff is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Fluff is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Fluff.  If not, see <https://www.gnu.org/licenses/>.
 */

pub struct Note {
    title: Option<String>,
    content: String
}

impl Note {
    pub fn new(title: Option<String>, content: String) -> Self {
        Self {
            title,
            content
        }
    }
}

#[cfg(test)]
mod test_note {
    use super::*;
    use test_case::test_case;

    #[test_case(None, "Hello, World!".to_string(); "should take none title")]
    #[test_case(Some("Title".to_string()), "Hello, World!".to_string(); "should take some title")]
    fn should_have_new_method(title: Option<String>, content: String) {
        let note = Note::new(title.clone(), content.clone());
        assert_eq!(note.title, title);
        assert_eq!(note.content, content);
    }
}