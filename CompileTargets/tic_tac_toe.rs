let u16 MapX = 3;
let u16 MapY = 3;


let int[,]map = {
    {0;0;0;};
    {0;0;0;};
    {0;0;0;};
};
// [ ][o][x]
// [ ][o][x]
// [o][ ][ ]
fn DrawMap(){
    Console.WriteLine("");
    let str wholeMap = "";
    for y in 0..MapY{
        let str line = "";
        for x in 0..MapX{
 	    let i16 playerIndex = map[x,y];
            let str playerSymbol =  " "; 

            if ( playerIndex == 1 ){
                playerSymbol= "x";
            }else if (playerIndex == 2){
                playerSymbol= "o";
            }
            line += $"[{playerSymbol}]";
        }
        wholeMap += line + " \n";
    }

    Console.WriteLine(wholeMap);
    Console.WriteLine("");
}
fn IsPosValid(u32 x, u32 y ) -> bool{
    return x < MapX && y < MapY && map[x,y] == 0;
}

Console.WriteLine("Let's start the game!");

DrawMap();

let bool xNowPlaying = true;
while (true){
    let str player_symbol = "x";
    if ( xNowPlaying ){
        player_symbol = "o";
    }
    Console.WriteLine($"Current player:{player_symbol}");
 
    Console.WriteLine("");
    Console.WriteLine("Type row");
    if ( !uint.TryParse( Console.ReadLine(),out uint row) ){
        Console.WriteLine("input is invalid");
        continue;
    }
    Console.WriteLine("");
    Console.WriteLine("Type column");
    if ( !uint.TryParse( Console.ReadLine(),out uint column) ){
        Console.WriteLine("input is invalid");
        continue;
    }
    if ( !IsPosValid(column,row) ){
        Console.WriteLine("position is invalid");
        continue;
    }
    if (xNowPlaying){
        map[column,row] = 1;
    }else{
        map[column,row] = 2;
    }
    DrawMap();
    xNowPlaying = !xNowPlaying;
}
