from random import choice

plays = range (1,10)
played = set()

while len(played) < 9:
    play = choice(plays)
    played.add(play)
    print(play)
