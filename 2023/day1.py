import regex

with open('data/day1.txt') as file:
    sum = 0
    for line in file:

        value = ''

        number = regex.search(r'one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9', line)
        if number:
            if not number.group().isdigit():
                match number.group():
                    case 'one':
                        value += '1'
                    case 'two':
                        value += '2'
                    case 'three':
                        value += '3'
                    case 'four':
                        value += '4'
                    case 'five':
                        value += '5'
                    case 'six':
                        value += '6'
                    case 'seven':
                        value += '7'
                    case 'eight':
                        value += '8'
                    case 'nine':
                        value += '9'
            else:
                value += number.group()

        number = regex.search(r'one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9', line, regex.REVERSE)
        if number:
            if not number.group().isdigit():
                match number.group():
                    case 'one':
                        value += '1'
                    case 'two':
                        value += '2'
                    case 'three':
                        value += '3'
                    case 'four':
                        value += '4'
                    case 'five':
                        value += '5'
                    case 'six':
                        value += '6'
                    case 'seven':
                        value += '7'
                    case 'eight':
                        value += '8'
                    case 'nine':
                        value += '9'
            else:
                value += number.group()

        print(value)

        sum += int(value)
    print(sum)
