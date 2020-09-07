pub mod pieces{
    #[derive(Debug, Copy, Clone)]
    pub enum Color{
        Black,
        White,
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Piece{
        pub color: Color,
    }
}

pub mod board{
    use super::pieces; 

    #[derive(Debug, Copy, Clone)]
    pub struct Square{
        piece: pieces::Piece,
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Board{
        grid: [[Square; 8]; 8],
    }

    impl Board{
        pub fn init() -> Board{
            let empty_sqare = Square{
                piece: pieces::Piece{
                    color: pieces::Color::Black,
                },
            };

            Board{
                grid: [[empty_sqare; 8]; 8],
            }
        }

        pub fn print_board(&self){
            for i in 0..8{
                for j in 0..8{
                    print!("{:?} ", self.grid[i][j].piece.color); 
                }
                println!(""); 
            }
            //write!("Board_number: {}", self.grid);
        }
    }
}