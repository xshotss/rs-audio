/*!
The effects module is useful for applying various effects to songs, without doing them manually.<br>
**Note**: This feature is still underdevelopment.
*/

use crate::{Note, Song};

pub fn echo(song: &Song, delay: f64, decay: f32) -> Song {
    let mut echoed_notes = Vec::new();

    for note in &song.notes {
        echoed_notes.push(note.clone());

        // Add echo
        if note.dur > delay {
            let echo_note = Note {
                vol: note.vol * decay,
                dur: note.dur - delay,
                ..*note
            };
            echoed_notes.push(echo_note);
        }
    }

    Song::new(echoed_notes, song.bpm)
}

pub fn reverse(song: &Song) -> Song {
    let mut reversed_notes = song.notes.clone();
    reversed_notes.reverse();
    Song::new(reversed_notes, song.bpm)
}

pub fn speed_up(song: &Song, factor: f64) -> Song {
    let speed_notes: Vec<Note> = song
        .notes
        .iter()
        .map(|note| Note {
            dur: note.dur / factor,
            ..*note
        })
        .collect();

    Song::new(speed_notes, song.bpm)
}
