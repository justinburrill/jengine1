if __name__ == "__main__":
    letters = [chr(i) for i in range(ord("A"), ord("H") + 1)]
    numbers = list(range(1, 9))
    squares = [f"{l}{n}" for n in numbers for l in letters]
    for i, sq in enumerate(squares):
        print(f"{sq} = {i},")
