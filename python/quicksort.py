import random
import datetime
def partitionAsc(arr, low, high):
 i = low - 1 # index of smaller element
 pivot = arr[high] # pivot: using last element as pivot
 for j in range(low, high): #j = 0,1,2,..,7
  # If current element is smaller than or equal to pivot
  if arr[j] <= pivot:
   # increment index of smaller element
   i = i+1
   arr[i], arr[j] = arr[j], arr[i] #swaps arr[i] and arr[j]
 
 arr[i+1], arr[high] = arr[high], arr[i+1] #swaps
 return i+1 #index of the partition that is in its correct position
def quickSortAsc(arr, low, high):
 if low < high:
  # pi is partitioning index, arr[p] is now at right place
  pi = partitionAsc(arr, low, high)
  # Separately sort elements before partition and after partition
  quickSortAsc(arr, low, pi-1)
  quickSortAsc(arr, pi+1, high)
 return

if __name__ == "__main__":
 array = [index for index in range(1, 100001)]
 random.shuffle(array)
 start = datetime.datetime.now()
 quickSortAsc(array, 0, len(array)-1)
 end = datetime.datetime.now()
 print("It took: ", (end-start).total_seconds()*1000,  "ms to sort")