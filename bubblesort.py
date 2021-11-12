import random
import datetime
def BubbleSortAsc(array):
 no_swap = True
 
 while no_swap:
  no_swap = False
  for position in range(0, len(array)-1):
   if array[position] > array[position+1]:
    # swap
    temp = array[position]
    array[position] = array[position+1]
    array[position+1] = temp
    no_swap = True

if __name__ == "__main__":
 array = [index for index in range(1, 10001)]
 random.shuffle(array)
 start = datetime.datetime.now()
 BubbleSortAsc(array)
 end = datetime.datetime.now()
 print("It took: ", (end-start).total_seconds()*1000, "ms to sort")