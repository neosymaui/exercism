def abbreviate(words):
    replace = ['-']
    remove = ['_']

    for rep in replace:
        words = words.replace(rep, " ")
    for rem in remove:
        words = words.replace(rem, "")

    return "".join([word[0].upper() for word in words.split()])
