use std::fmt::Display;

const BSIZE: usize = 20;

pub struct Board {
    boats: [u8; 4],
    data: [[u8; BSIZE]; BSIZE],
}

pub enum Error {
    Overlap,
    OutOfBounds,
    BoatCount,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Overlap => write!(f, "Overlap"),
            Self::OutOfBounds => write!(f, "OutOfBounds"),
            Self::BoatCount => write!(f, "BoatCount"),
        }
    }
}

pub enum Boat {
    Vertical(usize),
    Horizontal(usize),
}

impl Board {
    /** crea una board vuota con una disponibilità di navi */
    pub fn new(boats: &[u8]) -> Board {
        Board {
            boats: [boats[0], boats[1], boats[2], boats[3]],
            data: [[0; BSIZE]; BSIZE],
        }
    }

    /* crea una board a partire da una stringa che rappresenta tutto
    il contenuto del file board.txt */
    pub fn from(s: String) -> Board {
        let lines: Vec<&str> = s.split('\n').collect();
        let mut boats = [0_u8; 4];
        let mut data = [[0; BSIZE]; BSIZE];

        for i in 0..4 {
            boats[i] = lines[0].chars().nth(2 * i).unwrap_or('0') as u8 - '0' as u8
        }

        for line_num in 0..BSIZE {
            let line: Vec<char> = lines[line_num + 1].chars().collect();
            for el in 0..BSIZE {
                if line[el] == 'B' {
                    data[line_num][el] = 1;
                }
            }
        }

        Board { boats, data }
    }

    /* aggiunge la nave alla board, restituendo la nuova board se
    possibile */
    /* bonus: provare a *non copiare* data quando si crea e restituisce
    una nuova board con la barca, come si può fare? */
    pub fn add_boat(self, boat: Boat, pos: (usize, usize)) -> Result<Board, Error> {
        let len: usize;
        let horiz: bool;

        let mut res = self;

        match boat {
            Boat::Horizontal(l) => {
                horiz = true;
                len = l;
            }
            Boat::Vertical(l) => {
                horiz = false;
                len = l;
            }
        };

        let hor_factor = if horiz { 1 } else { 0 };
        let ver_factor = 1 - hor_factor;

        if res.boats[len - 1] == 0 {
            return Err(Error::BoatCount);
        }

        res.boats[len - 1] -= 1;

        let mut border: Vec<(isize, isize)> = Vec::with_capacity(2 * len + 2);
        let pos: (isize, isize) = (pos.0 as isize, pos.1 as isize);

        for delta in 0_isize..len as isize {
            let row = pos.0 + delta * ver_factor - 1;
            let col = pos.1 + delta * hor_factor - 1;

            if row >= BSIZE as isize || col >= BSIZE as isize {
                return Err(Error::OutOfBounds);
            }

            if res.data[row as usize][col as usize] == 1 {
                return Err(Error::Overlap);
            }

            res.data[row as usize][col as usize] = 1;

            border.push((row + hor_factor, col + ver_factor));
            border.push((row - hor_factor, col - ver_factor));
        }

        border.push((pos.0 - ver_factor - 1, pos.1 - hor_factor - 1));
        border.push((
            pos.0 + ver_factor * (len as isize + ver_factor) - 1,
            pos.1 + hor_factor * (len as isize + hor_factor) - 1,
        ));

        for (row, col) in border {
            if row < 0 || row >= BSIZE as isize || col < 0 || col >= BSIZE as isize {
                continue;
            }

            if res.data[row as usize][col as usize] == 1 {
                return Err(Error::Overlap);
            }
        }

        Ok(res)
    }

    /* converte la board in una stringa salvabile su file */
    pub fn to_string(&self) -> String {
        let mut tmp = String::new();

        for i in 0..4 {
            let num = char::from_digit(self.boats[i] as u32, 10);
            tmp.push(num.unwrap_or('0'));
            tmp.push(if i == 3 { '\n' } else { ' ' });
        }

        for row in self.data {
            for cell in row {
                tmp.push(if cell == 0 { ' ' } else { 'B' });
            }
            tmp.push('\n');
        }

        tmp
    }
}
