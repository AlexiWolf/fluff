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

#[cfg(test)]
mod test_notes {
    use super::*;

    const TITLE: Option<String> = Some("Title".to_string());
    const CONTENT: String = "Hello, World!".to_string();
    const NOTE: Note = Note {title: TITLE, content: CONTENT};

    #[test]
    fn should_have_new_method() {
        let note = Note::new(TITLE, CONTENT);
        assert_eq!(note.title, NOTE);
        assert_eq!(note.content, CONTENT);
    }
}