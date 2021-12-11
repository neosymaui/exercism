def score(word):
    points = {
        "A": 1,
        "Z": 10,
        "E": 1,
        "R": 1,
        "T": 1,
        "Y": 4,
        "U": 1,
        "I": 1,
        "O": 1,
        "P": 3,
        "Q": 10,
        "S": 1,
        "D": 2,
        "F": 4,
        "G": 2,
        "H": 4,
        "J": 8,
        "K": 5,
        "L": 1,
        "M": 3,
        "W": 4,
        "X": 8,
        "C": 3,
        "V": 4,
        "B": 3,
        "N": 1
    }

    return sum([points[c] for c in word.upper()])
