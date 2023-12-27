import tkinter as tk
from pygame import mixer


class SoundboardApp:
    def __init__(self, master):
        self.master = master
        self.master.title("Soundboard")

        # init mixer
        mixer.init()

        # dymaically create buttons in board
        self.create_dynamic_sound_buttons()



    def create_dynamic_sound_buttons(self):
        # TODO get data via API
        for button in api_data:
            self.button1 = tk.Button(self.master, text="Sound 1", command=lambda: self.play_sound("sound1.wav"))
            self.button1.pack(pady=10)

        # Create quit button
        self.quit_button = tk.Button(self.master, text="Beenden", command=self.master.destroy)
        self.quit_button.pack(pady=20)

    def play_sound(self, sound_file):
        # TODO exchange mixer.music.load with API call to play sound
        mixer.music.load(sound_file)
        mixer.music.play()


if __name__ == "__main__":
    # TODO set fixed main window size
    root = tk.Tk()
    app = SoundboardApp(root)
    root.mainloop()
