def main():
    num_passwords = 0
    for i in range(134564, 585159 + 1):
        if check_if_ok(i):
            print(i)
            num_passwords += 1
    print(num_passwords)


def check_if_ok(password):
    string_pass = str(password)
    current_letter = ""
    range_len = 0
    if "".join(sorted(string_pass)) == string_pass:
        for i, c in enumerate(string_pass):
            if c == current_letter:
                range_len += 1
            else:
                if range_len == 1:
                    return True
                range_len = 0
            if range_len == 1 and i == len(string_pass) - 1:
                return True
            current_letter = c
    return False


main()
