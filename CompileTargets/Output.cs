uint MapX = 3;
uint MapY = 3;
int[,] map = {{0, 0, 0}, {0, 0, 0}, {0, 0, 0}};
void DrawMap(){
	Console.WriteLine("");
	string wholeMap = "";
	for(int y = 0; y < MapY; y++) {
		string line = "";
		for(int x = 0; x < MapX; x++) {
			int playerIndex = map[x, y];
			string playerSymbol = " ";
			if((playerIndex == 1)){
				playerSymbol = "x";
			}
			else if((playerIndex == 2)){
			playerSymbol = "o";
			}
			line += $"[{playerSymbol}]";
		}
		wholeMap += (line + " \n");
	}
	Console.WriteLine(wholeMap);
	Console.WriteLine("");
} bool IsPosValid(ulong x, ulong y){
	return (((x < MapX) && (y < MapY)) && (map[x, y] == 0));
}
Console.WriteLine("Let's start the game!");
DrawMap();
bool xNowPlaying = true;
while(true){
	string player_symbol = "x";
	if(xNowPlaying){
		player_symbol = "o";
	}
	Console.WriteLine($"Current player:{player_symbol}");
	Console.WriteLine("");
	Console.WriteLine("Type row");
	if(!uint.TryParse(Console.ReadLine(), out uint row)){
		Console.WriteLine("input is invalid");
		continue;
	}
	Console.WriteLine("");
	Console.WriteLine("Type column");
	if(!uint.TryParse(Console.ReadLine(), out uint column)){
		Console.WriteLine("input is invalid");
		continue;
	}
	if(!IsPosValid(column, row)){
		Console.WriteLine("position is invalid");
		continue;
	}
	if(xNowPlaying){
		map[column, row] = 1;
	}
	else {
		map[column, row] = 2;
	}
	DrawMap();
	xNowPlaying = !xNowPlaying;
}
