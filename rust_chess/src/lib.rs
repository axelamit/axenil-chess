pub mod pieces{
    //Individual pieces 
    #[derive(Debug, Copy, Clone)]
    pub struct Pawn{}
    pub struct Bishop{}
    pub struct Knight{}
    pub struct Rook{}
    pub struct Queen{}
    pub struct King{}
}

pub mod units{
    #[derive(Debug, Copy, Clone)]
    pub struct Piece{
        pub type: Type,
        pub color: Color,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum Type{
        Empty,
        Pawn, 
        Bishop,
        Knight,
        Rook, 
        Queen,
        King,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum Color{
        Empty, 
        Black,
        White,
    }
}

pub mod board{
    use super::units; 

    #[derive(Debug, Copy, Clone)]
    pub struct Square{
        piece: units::Piece,
    }

    impl Square{
        fn is_empty(&self) -> bool{
            match self.piece.type{
                units::Type::Empty => true,
                _ => false,
            }
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Board{
        grid: [[Square; 8]; 8],
    }

    impl Board{
        pub fn init() -> Board{
            let empty_sqare = Square{
                piece: units::Piece{
                    type: units::Type::Empty, 
                    color: units::Color::Empty,
                },
            };

            println!("{:?}", empty_sqare.is_empty()); 

            Board{
                grid: [[empty_sqare; 8]; 8],
            }
        }

        pub fn print_board(&self){
            for i in 0..8{
                for j in 0..8{
                    print!("{:?} ", self.grid[i][j].piece.type);
                }
                println!(""); 
            }
        }
    }
}