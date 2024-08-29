import pygame
import time

def draw_data(win, data, highlight=-1):
    win.fill((255,255,255))
    for index in range(len(data)):
        if index == highlight:
            color = (255,255,0)
        else:
            color = (100,0,0)
        pygame.draw.rect(win, color, (index*10,500-data[index],10,data[index]))
    pygame.display.update()
    time.sleep(0.001)