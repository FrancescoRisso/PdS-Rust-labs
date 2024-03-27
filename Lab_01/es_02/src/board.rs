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

        if res.boats[len] == 0 {
            return Err(Error::BoatCount);
        }

        for delta in 0..len {
            let pos_with_delta = (
                pos.0 + delta * ver_factor - 1,
                pos.1 + delta * hor_factor - 1,
            );

            if pos_with_delta.0 >= BSIZE || pos_with_delta.1 >= BSIZE {
                return Err(Error::OutOfBounds);
            }

            if res.data[pos_with_delta.0][pos_with_delta.1] == 1
                || (horiz && pos.0 != 0 && res.data[pos.0 - 1][pos_with_delta.1] == 1)
                || (horiz && pos.0 != BSIZE - 1 && res.data[pos.0 + 1][pos_with_delta.1] == 1)
                || (!horiz && pos.1 != 0 && res.data[pos_with_delta.0][pos.1 - 1] == 1)
                || (!horiz && pos.1 != BSIZE - 1 && res.data[pos_with_delta.0][pos.1 + 1] == 1)
            {
                return Err(Error::Overlap);
            }

            res.data[pos_with_delta.0][pos_with_delta.1] = 1;
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
