zeroCount = 0
currentSafeValue = 50
changeValue = 0

with open("instructions.txt", "r", encoding="utf-8") as textFile:
          for line in textFile:
            line = line.rstrip("\n")
            print(line)
            if (line.find("R") != -1):
                print("Right")
                changeValue = int(line.removeprefix("R")) % 100
                print("Change Value =", changeValue)
                currentSafeValue = currentSafeValue - changeValue
                print("New Value = ", currentSafeValue, "\n")
            
            elif(line.find("L") != -1):
                print("Left")
                changeValue = int(line.removeprefix("L")) % 100
                print("Change Value = ", changeValue)
                currentSafeValue = currentSafeValue + changeValue
                print("New Value = ", currentSafeValue, "\n")

            else:
                print("Failed!!")

            if (currentSafeValue == 0):
                zeroCount = zeroCount + 1
                print("Add One to Zero Count")

            print("Zero Count = ", zeroCount)
