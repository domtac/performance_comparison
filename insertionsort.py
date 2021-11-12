import random
import datetime
def InsertionSortAsc(array):
 for index in range(1, len(array)):
 
  key = array[index]
  j = index - 1
  while j >=0 and key < array[j]:
   array[j+1] = array[j]
   j -= 1
 
  array[j+1] = key

if __name__ == "__main__":
 array = [index for index in range(1, 10001)]
 random.shuffle(array)
 start = datetime.datetime.now()
 InsertionSortAsc(array)
 end = datetime.datetime.now()
 print("It took: ", (end-start).total_seconds()*1000, "ms to sort")