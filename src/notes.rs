use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NoteLetter {
	C,
	D,
	E,
	F,
	G,
	A,
	B,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Note {
	pub note_letter: NoteLetter,
	pub sharp: bool,
	pub octave: i8,

	/// In Hz
	pub frequency: f32,
}

impl Display for Note {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{:?}{}{}",
			self.note_letter,
			if self.sharp { "#" } else { "" },
			self.octave
		)
	}
}

impl Note {
	pub const CN1: Note = Note {
		note_letter: NoteLetter::C,
		sharp: false,
		octave: -1,
		frequency: 8.175,
	};
	pub const CSN1: Note = Note {
		note_letter: NoteLetter::C,
		sharp: true,
		octave: -1,
		frequency: 8.661,
	};
	pub const DN1: Note = Note {
		note_letter: NoteLetter::D,
		sharp: false,
		octave: -1,
		frequency: 9.176,
	};
	pub const DSN1: Note = Note {
		note_letter: NoteLetter::D,
		sharp: true,
		octave: -1,
		frequency: 9.722,
	};
	pub const EN1: Note = Note {
		note_letter: NoteLetter::E,
		sharp: false,
		octave: -1,
		frequency: 10.30,
	};
	pub const FN1: Note = Note {
		note_letter: NoteLetter::F,
		sharp: false,
		octave: -1,
		frequency: 10.91,
	};
	pub const FSN1: Note = Note {
		note_letter: NoteLetter::F,
		sharp: true,
		octave: -1,
		frequency: 11.56,
	};
	pub const GN1: Note = Note {
		note_letter: NoteLetter::G,
		sharp: false,
		octave: -1,
		frequency: 12.25,
	};
	pub const GSN1: Note = Note {
		note_letter: NoteLetter::G,
		sharp: true,
		octave: -1,
		frequency: 12.98,
	};
	pub const AN1: Note = Note {
		note_letter: NoteLetter::A,
		sharp: false,
		octave: -1,
		frequency: 13.75,
	};
	pub const ASN1: Note = Note {
		note_letter: NoteLetter::A,
		sharp: true,
		octave: -1,
		frequency: 14.57,
	};
	pub const BN1: Note = Note {
		note_letter: NoteLetter::B,
		sharp: false,
		octave: -1,
		frequency: 15.43,
	};
	pub const C0: Note = Note {
		note_letter: NoteLetter::C,
		sharp: false,
		octave: 0,
		frequency: 16.35,
	};
	pub const CS0: Note = Note {
		note_letter: NoteLetter::C,
		sharp: true,
		octave: 0,
		frequency: 17.32,
	};
	pub const D0: Note = Note {
		note_letter: NoteLetter::D,
		sharp: false,
		octave: 0,
		frequency: 18.35,
	};
	pub const DS0: Note = Note {
		note_letter: NoteLetter::D,
		sharp: true,
		octave: 0,
		frequency: 19.45,
	};
	pub const E0: Note = Note {
		note_letter: NoteLetter::E,
		sharp: false,
		octave: 0,
		frequency: 20.60,
	};
	pub const F0: Note = Note {
		note_letter: NoteLetter::F,
		sharp: false,
		octave: 0,
		frequency: 21.83,
	};
	pub const FS0: Note = Note {
		note_letter: NoteLetter::F,
		sharp: true,
		octave: 0,
		frequency: 23.12,
	};
	pub const G0: Note = Note {
		note_letter: NoteLetter::G,
		sharp: false,
		octave: 0,
		frequency: 24.50,
	};
	pub const GS0: Note = Note {
		note_letter: NoteLetter::G,
		sharp: true,
		octave: 0,
		frequency: 25.96,
	};
	pub const A0: Note = Note {
		note_letter: NoteLetter::A,
		sharp: false,
		octave: 0,
		frequency: 27.50,
	};
	pub const AS0: Note = Note {
		note_letter: NoteLetter::A,
		sharp: true,
		octave: 0,
		frequency: 29.14,
	};
	pub const B0: Note = Note {
		note_letter: NoteLetter::B,
		sharp: false,
		octave: 0,
		frequency: 30.87,
	};
	pub const C1: Note = Note {
		note_letter: NoteLetter::C,
		sharp: false,
		octave: 1,
		frequency: 32.70,
	};
	pub const CS1: Note = Note {
		note_letter: NoteLetter::C,
		sharp: true,
		octave: 1,
		frequency: 34.65,
	};
	pub const D1: Note = Note {
		note_letter: NoteLetter::D,
		sharp: false,
		octave: 1,
		frequency: 36.71,
	};
	pub const DS1: Note = Note {
		note_letter: NoteLetter::D,
		sharp: true,
		octave: 1,
		frequency: 38.89,
	};
	pub const E1: Note = Note {
		note_letter: NoteLetter::E,
		sharp: false,
		octave: 1,
		frequency: 41.20,
	};
	pub const F1: Note = Note {
		note_letter: NoteLetter::F,
		sharp: false,
		octave: 1,
		frequency: 43.65,
	};
	pub const FS1: Note = Note {
		note_letter: NoteLetter::F,
		sharp: true,
		octave: 1,
		frequency: 46.25,
	};
	pub const G1: Note = Note {
		note_letter: NoteLetter::G,
		sharp: false,
		octave: 1,
		frequency: 49.00,
	};
	pub const GS1: Note = Note {
		note_letter: NoteLetter::G,
		sharp: true,
		octave: 1,
		frequency: 51.91,
	};
	pub const A1: Note = Note {
		note_letter: NoteLetter::A,
		sharp: false,
		octave: 1,
		frequency: 55.00,
	};
	pub const AS1: Note = Note {
		note_letter: NoteLetter::A,
		sharp: true,
		octave: 1,
		frequency: 58.27,
	};
	pub const B1: Note = Note {
		note_letter: NoteLetter::B,
		sharp: false,
		octave: 1,
		frequency: 61.74,
	};
	pub const C2: Note = Note {
		note_letter: NoteLetter::C,
		sharp: false,
		octave: 2,
		frequency: 65.41,
	};
	pub const CS2: Note = Note {
		note_letter: NoteLetter::C,
		sharp: true,
		octave: 2,
		frequency: 69.30,
	};
	pub const D2: Note = Note {
		note_letter: NoteLetter::D,
		sharp: false,
		octave: 2,
		frequency: 73.42,
	};
	pub const DS2: Note = Note {
		note_letter: NoteLetter::D,
		sharp: true,
		octave: 2,
		frequency: 77.78,
	};
	pub const E2: Note = Note {
		note_letter: NoteLetter::E,
		sharp: false,
		octave: 2,
		frequency: 82.41,
	};
	pub const F2: Note = Note {
		note_letter: NoteLetter::F,
		sharp: false,
		octave: 2,
		frequency: 87.31,
	};
	pub const FS2: Note = Note {
		note_letter: NoteLetter::F,
		sharp: true,
		octave: 2,
		frequency: 92.50,
	};
	pub const G2: Note = Note {
		note_letter: NoteLetter::G,
		sharp: false,
		octave: 2,
		frequency: 98.00,
	};
	pub const GS2: Note = Note {
		note_letter: NoteLetter::G,
		sharp: true,
		octave: 2,
		frequency: 103.83,
	};
	pub const A2: Note = Note {
		note_letter: NoteLetter::A,
		sharp: false,
		octave: 2,
		frequency: 110.00,
	};
	pub const AS2: Note = Note {
		note_letter: NoteLetter::A,
		sharp: true,
		octave: 2,
		frequency: 116.54,
	};
	pub const B2: Note = Note {
		note_letter: NoteLetter::B,
		sharp: false,
		octave: 2,
		frequency: 123.47,
	};
	pub const C3: Note = Note {
		note_letter: NoteLetter::C,
		sharp: false,
		octave: 3,
		frequency: 130.81,
	};
	pub const CS3: Note = Note {
		note_letter: NoteLetter::C,
		sharp: true,
		octave: 3,
		frequency: 138.59,
	};
	pub const D3: Note = Note {
		note_letter: NoteLetter::D,
		sharp: false,
		octave: 3,
		frequency: 146.83,
	};
	pub const DS3: Note = Note {
		note_letter: NoteLetter::D,
		sharp: true,
		octave: 3,
		frequency: 155.56,
	};
	pub const E3: Note = Note {
		note_letter: NoteLetter::E,
		sharp: false,
		octave: 3,
		frequency: 164.81,
	};
	pub const F3: Note = Note {
		note_letter: NoteLetter::F,
		sharp: false,
		octave: 3,
		frequency: 174.61,
	};
	pub const FS3: Note = Note {
		note_letter: NoteLetter::F,
		sharp: true,
		octave: 3,
		frequency: 185.00,
	};
	pub const G3: Note = Note {
		note_letter: NoteLetter::G,
		sharp: false,
		octave: 3,
		frequency: 196.00,
	};
	pub const GS3: Note = Note {
		note_letter: NoteLetter::G,
		sharp: true,
		octave: 3,
		frequency: 207.65,
	};
	pub const A3: Note = Note {
		note_letter: NoteLetter::A,
		sharp: false,
		octave: 3,
		frequency: 220.00,
	};
	pub const AS3: Note = Note {
		note_letter: NoteLetter::A,
		sharp: true,
		octave: 3,
		frequency: 233.08,
	};
	pub const B3: Note = Note {
		note_letter: NoteLetter::B,
		sharp: false,
		octave: 3,
		frequency: 246.94,
	};
	pub const C4: Note = Note {
		note_letter: NoteLetter::C,
		sharp: false,
		octave: 4,
		frequency: 261.63,
	};
	pub const CS4: Note = Note {
		note_letter: NoteLetter::C,
		sharp: true,
		octave: 4,
		frequency: 277.18,
	};
	pub const D4: Note = Note {
		note_letter: NoteLetter::D,
		sharp: false,
		octave: 4,
		frequency: 293.66,
	};
	pub const DS4: Note = Note {
		note_letter: NoteLetter::D,
		sharp: true,
		octave: 4,
		frequency: 311.13,
	};
	pub const E4: Note = Note {
		note_letter: NoteLetter::E,
		sharp: false,
		octave: 4,
		frequency: 329.63,
	};
	pub const F4: Note = Note {
		note_letter: NoteLetter::F,
		sharp: false,
		octave: 4,
		frequency: 349.23,
	};
	pub const FS4: Note = Note {
		note_letter: NoteLetter::F,
		sharp: true,
		octave: 4,
		frequency: 369.99,
	};
	pub const G4: Note = Note {
		note_letter: NoteLetter::G,
		sharp: false,
		octave: 4,
		frequency: 392.00,
	};
	pub const GS4: Note = Note {
		note_letter: NoteLetter::G,
		sharp: true,
		octave: 4,
		frequency: 415.30,
	};
	pub const A4: Note = Note {
		note_letter: NoteLetter::A,
		sharp: false,
		octave: 4,
		frequency: 440.00,
	};
	pub const AS4: Note = Note {
		note_letter: NoteLetter::A,
		sharp: true,
		octave: 4,
		frequency: 466.16,
	};
	pub const B4: Note = Note {
		note_letter: NoteLetter::B,
		sharp: false,
		octave: 4,
		frequency: 493.88,
	};
	pub const C5: Note = Note {
		note_letter: NoteLetter::C,
		sharp: false,
		octave: 5,
		frequency: 523.25,
	};
	pub const CS5: Note = Note {
		note_letter: NoteLetter::C,
		sharp: true,
		octave: 5,
		frequency: 554.37,
	};
	pub const D5: Note = Note {
		note_letter: NoteLetter::D,
		sharp: false,
		octave: 5,
		frequency: 587.33,
	};
	pub const DS5: Note = Note {
		note_letter: NoteLetter::D,
		sharp: true,
		octave: 5,
		frequency: 622.25,
	};
	pub const E5: Note = Note {
		note_letter: NoteLetter::E,
		sharp: false,
		octave: 5,
		frequency: 659.25,
	};
	pub const F5: Note = Note {
		note_letter: NoteLetter::F,
		sharp: false,
		octave: 5,
		frequency: 698.46,
	};
	pub const FS5: Note = Note {
		note_letter: NoteLetter::F,
		sharp: true,
		octave: 5,
		frequency: 739.99,
	};
	pub const G5: Note = Note {
		note_letter: NoteLetter::G,
		sharp: false,
		octave: 5,
		frequency: 783.99,
	};
	pub const GS5: Note = Note {
		note_letter: NoteLetter::G,
		sharp: true,
		octave: 5,
		frequency: 830.61,
	};
	pub const A5: Note = Note {
		note_letter: NoteLetter::A,
		sharp: false,
		octave: 5,
		frequency: 880.00,
	};
	pub const AS5: Note = Note {
		note_letter: NoteLetter::A,
		sharp: true,
		octave: 5,
		frequency: 932.33,
	};
	pub const B5: Note = Note {
		note_letter: NoteLetter::B,
		sharp: false,
		octave: 5,
		frequency: 987.77,
	};
	pub const C6: Note = Note {
		note_letter: NoteLetter::C,
		sharp: false,
		octave: 6,
		frequency: 1046.50,
	};
	pub const CS6: Note = Note {
		note_letter: NoteLetter::C,
		sharp: true,
		octave: 6,
		frequency: 1108.73,
	};
	pub const D6: Note = Note {
		note_letter: NoteLetter::D,
		sharp: false,
		octave: 6,
		frequency: 1174.66,
	};
	pub const DS6: Note = Note {
		note_letter: NoteLetter::D,
		sharp: true,
		octave: 6,
		frequency: 1244.51,
	};
	pub const E6: Note = Note {
		note_letter: NoteLetter::E,
		sharp: false,
		octave: 6,
		frequency: 1318.51,
	};
	pub const F6: Note = Note {
		note_letter: NoteLetter::F,
		sharp: false,
		octave: 6,
		frequency: 1396.91,
	};
	pub const FS6: Note = Note {
		note_letter: NoteLetter::F,
		sharp: true,
		octave: 6,
		frequency: 1479.98,
	};
	pub const G6: Note = Note {
		note_letter: NoteLetter::G,
		sharp: false,
		octave: 6,
		frequency: 1567.98,
	};
	pub const GS6: Note = Note {
		note_letter: NoteLetter::G,
		sharp: true,
		octave: 6,
		frequency: 1661.22,
	};
	pub const A6: Note = Note {
		note_letter: NoteLetter::A,
		sharp: false,
		octave: 6,
		frequency: 1760.00,
	};
	pub const AS6: Note = Note {
		note_letter: NoteLetter::A,
		sharp: true,
		octave: 6,
		frequency: 1864.66,
	};
	pub const B6: Note = Note {
		note_letter: NoteLetter::B,
		sharp: false,
		octave: 6,
		frequency: 1975.53,
	};
	pub const C7: Note = Note {
		note_letter: NoteLetter::C,
		sharp: false,
		octave: 7,
		frequency: 2093.00,
	};
	pub const CS7: Note = Note {
		note_letter: NoteLetter::C,
		sharp: true,
		octave: 7,
		frequency: 2217.46,
	};
	pub const D7: Note = Note {
		note_letter: NoteLetter::D,
		sharp: false,
		octave: 7,
		frequency: 2349.83,
	};
	pub const DS7: Note = Note {
		note_letter: NoteLetter::D,
		sharp: true,
		octave: 7,
		frequency: 2489.02,
	};
	pub const E7: Note = Note {
		note_letter: NoteLetter::E,
		sharp: false,
		octave: 7,
		frequency: 2637.02,
	};
	pub const F7: Note = Note {
		note_letter: NoteLetter::F,
		sharp: false,
		octave: 7,
		frequency: 2793.83,
	};
	pub const FS7: Note = Note {
		note_letter: NoteLetter::F,
		sharp: true,
		octave: 7,
		frequency: 2959.96,
	};
	pub const G7: Note = Note {
		note_letter: NoteLetter::G,
		sharp: false,
		octave: 7,
		frequency: 3135.96,
	};
	pub const GS7: Note = Note {
		note_letter: NoteLetter::G,
		sharp: true,
		octave: 7,
		frequency: 3322.44,
	};
	pub const A7: Note = Note {
		note_letter: NoteLetter::A,
		sharp: false,
		octave: 7,
		frequency: 3520.00,
	};
	pub const AS7: Note = Note {
		note_letter: NoteLetter::A,
		sharp: true,
		octave: 7,
		frequency: 3729.31,
	};
	pub const B7: Note = Note {
		note_letter: NoteLetter::B,
		sharp: false,
		octave: 7,
		frequency: 3951.07,
	};
	pub const C8: Note = Note {
		note_letter: NoteLetter::C,
		sharp: false,
		octave: 8,
		frequency: 4186.01,
	};
	pub const CS8: Note = Note {
		note_letter: NoteLetter::C,
		sharp: true,
		octave: 8,
		frequency: 4434.92,
	};
	pub const D8: Note = Note {
		note_letter: NoteLetter::D,
		sharp: false,
		octave: 8,
		frequency: 4698.63,
	};
	pub const DS8: Note = Note {
		note_letter: NoteLetter::D,
		sharp: true,
		octave: 8,
		frequency: 4978.03,
	};
	pub const E8: Note = Note {
		note_letter: NoteLetter::E,
		sharp: false,
		octave: 8,
		frequency: 5274.04,
	};
	pub const F8: Note = Note {
		note_letter: NoteLetter::F,
		sharp: false,
		octave: 8,
		frequency: 5587.65,
	};
	pub const FS8: Note = Note {
		note_letter: NoteLetter::F,
		sharp: true,
		octave: 8,
		frequency: 5919.91,
	};
	pub const G8: Note = Note {
		note_letter: NoteLetter::G,
		sharp: false,
		octave: 8,
		frequency: 6271.93,
	};
	pub const GS8: Note = Note {
		note_letter: NoteLetter::G,
		sharp: true,
		octave: 8,
		frequency: 6644.88,
	};
	pub const A8: Note = Note {
		note_letter: NoteLetter::A,
		sharp: false,
		octave: 8,
		frequency: 7040.00,
	};
	pub const AS8: Note = Note {
		note_letter: NoteLetter::A,
		sharp: true,
		octave: 8,
		frequency: 7458.62,
	};
	pub const B8: Note = Note {
		note_letter: NoteLetter::B,
		sharp: false,
		octave: 8,
		frequency: 7902.13,
	};
	pub const C9: Note = Note {
		note_letter: NoteLetter::C,
		sharp: false,
		octave: 9,
		frequency: 8372.02,
	};
	pub const CS9: Note = Note {
		note_letter: NoteLetter::C,
		sharp: true,
		octave: 9,
		frequency: 8869.84,
	};
	pub const D9: Note = Note {
		note_letter: NoteLetter::D,
		sharp: false,
		octave: 9,
		frequency: 9397.27,
	};
	pub const DS9: Note = Note {
		note_letter: NoteLetter::D,
		sharp: true,
		octave: 9,
		frequency: 10548.1,
	};
	pub const E9: Note = Note {
		note_letter: NoteLetter::E,
		sharp: false,
		octave: 9,
		frequency: 11175.3,
	};
	pub const F9: Note = Note {
		note_letter: NoteLetter::F,
		sharp: false,
		octave: 9,
		frequency: 11839.8,
	};
	pub const FS9: Note = Note {
		note_letter: NoteLetter::F,
		sharp: true,
		octave: 9,
		frequency: 12543.8,
	};
	pub const G9: Note = Note {
		note_letter: NoteLetter::G,
		sharp: false,
		octave: 9,
		frequency: 13289.7,
	};

	pub const NOTES: [Note; 128] = [
		Note::CN1,
		Note::CSN1,
		Note::DN1,
		Note::DSN1,
		Note::EN1,
		Note::FN1,
		Note::FSN1,
		Note::GN1,
		Note::GSN1,
		Note::AN1,
		Note::ASN1,
		Note::BN1,
		Note::C0,
		Note::CS0,
		Note::D0,
		Note::DS0,
		Note::E0,
		Note::F0,
		Note::FS0,
		Note::G0,
		Note::GS0,
		Note::A0,
		Note::AS0,
		Note::B0,
		Note::C1,
		Note::CS1,
		Note::D1,
		Note::DS1,
		Note::E1,
		Note::F1,
		Note::FS1,
		Note::G1,
		Note::GS1,
		Note::A1,
		Note::AS1,
		Note::B1,
		Note::C2,
		Note::CS2,
		Note::D2,
		Note::DS2,
		Note::E2,
		Note::F2,
		Note::FS2,
		Note::G2,
		Note::GS2,
		Note::A2,
		Note::AS2,
		Note::B2,
		Note::C3,
		Note::CS3,
		Note::D3,
		Note::DS3,
		Note::E3,
		Note::F3,
		Note::FS3,
		Note::G3,
		Note::GS3,
		Note::A3,
		Note::AS3,
		Note::B3,
		Note::C4,
		Note::CS4,
		Note::D4,
		Note::DS4,
		Note::E4,
		Note::F4,
		Note::FS4,
		Note::G4,
		Note::GS4,
		Note::A4,
		Note::AS4,
		Note::B4,
		Note::C5,
		Note::CS5,
		Note::D5,
		Note::DS5,
		Note::E5,
		Note::F5,
		Note::FS5,
		Note::G5,
		Note::GS5,
		Note::A5,
		Note::AS5,
		Note::B5,
		Note::C6,
		Note::CS6,
		Note::D6,
		Note::DS6,
		Note::E6,
		Note::F6,
		Note::FS6,
		Note::G6,
		Note::GS6,
		Note::A6,
		Note::AS6,
		Note::B6,
		Note::C7,
		Note::CS7,
		Note::D7,
		Note::DS7,
		Note::E7,
		Note::F7,
		Note::FS7,
		Note::G7,
		Note::GS7,
		Note::A7,
		Note::AS7,
		Note::B7,
		Note::C8,
		Note::CS8,
		Note::D8,
		Note::DS8,
		Note::E8,
		Note::F8,
		Note::FS8,
		Note::G8,
		Note::GS8,
		Note::A8,
		Note::AS8,
		Note::B8,
		Note::C9,
		Note::CS9,
		Note::D9,
		Note::DS9,
		Note::E9,
		Note::F9,
		Note::FS9,
		Note::G9,
	];

	/// Relative to C-1 (the lowest midi note)
	pub fn position(&self) -> u8 {
		(self.octave * 7 + self.note_letter as i8) as u8
	}

	/// Relative to C-1 (the lowest midi note)
	pub fn from_position(position: u8) -> Self {
		Self::NOTES[position as usize]
	}
}
