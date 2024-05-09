def manhattan(point_from, point_to):
    return abs(point_from[0] - point_to[0]) + abs(point_from[1] - point_to[1])



def print_table(target):
    print("[")
    for y in range(16):
        print("[", end="")
        for x in range(16):
            result = (manhattan((x, y), target) / 3) - 5  
            if(x == 15):
                end = ""
            else:
                result_str = "{:.1f}".format(result) 
                if(len(result_str) < 4):
                    end = "  ,"
                else:
                    end = " ,"
            print(result_str, end = end)
        print("],")
    print("]")

print_table((15, 15))
print_table((0, 0))
