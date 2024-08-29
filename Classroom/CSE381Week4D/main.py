# CSE 381 REPL 4D Solved
# Sort Visualization

import random
import time
import pygame
from sorts import no_sort, selection_sort, insertion_sort, merge_sort, quick_sort, counting_sort

pygame.init()
win = pygame.display.set_mode((1000,500))
while True:
    data = [random.randint(0,499) for _ in range(100)]

    pygame.display.set_caption("No Sort")
    no_sort(win, data)
    time.sleep(2)

    pygame.display.set_caption("Selection Sort")
    selection_sort(win, data[:])
    time.sleep(2)

    pygame.display.set_caption("Insertion Sort")
    insertion_sort(win, data[:])
    time.sleep(2)

    pygame.display.set_caption("Merge Sort")
    merge_sort(win, data[:])
    time.sleep(2)

    pygame.display.set_caption("Quick Sort")
    quick_sort(win, data[:])
    time.sleep(2)

    pygame.display.set_caption("Counting Sort")
    counting_sort(win, data[:], 500)
    time.sleep(2)


