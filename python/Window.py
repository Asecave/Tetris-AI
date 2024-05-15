import pygame

class Window:

    def __init__():
        self.running = True

        screen = pygame.display.set_mode((300, 300))
        screen.fill(background_colour)
        pygame.display.set_caption('Tetris AI')
        pygame.display.flip()

        while running:
            for event in pygame.event.get():
                match event.TYPE:
                    case pygame.QUIT:
                        running = false


window = Window.__init__()
