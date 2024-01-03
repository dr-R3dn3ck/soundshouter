import tkinter as tk
import customtkinter
#from pygame import mixer
import requests

class SoundboardApp:
    def __init__(self, master):
        self.master = master
        self.master.title("Soundshouter V13.37")

        # init mixer
        #mixer.init()

        # Create Frames
        self.frame = customtkinter.CTkScrollableFrame(master=self.master, width=600, height=600)
        self.frame.place(relx=0.01, rely=0.1, anchor="nw")

        # request infos from openapi
        self.sounds = self.request_sounds()
        self.categories = self.request_categories()
        self.subcategories = self.request_subcategories()

        self.get_sounds()
        self.create_widgets()

    def get_sounds(self):
        self.request_sounds()

    def request_sounds(self):
        r = requests.get('http://127.0.0.1:5000/sounds')
        return r.json()

    def request_categories(self):
        r = requests.get('http://127.0.0.1:5000/categories')
        return r.json()

    def request_subcategories(self):
        r = requests.get('http://127.0.0.1:5000/subcategories')
        return r.json()

    def create_widgets(self):
        # Create Head label
        self.head_label = customtkinter.CTkLabel(self.master, text="Spielen du H...", fg_color="grey", width=100, height=50, text_color="blue", font=("Courier", 44))
        self.head_label.place(relx=0.01, rely=0.02, anchor="nw")

        # Create quit button
        self.quit_button = tk.Button(self.master, text="Beenden", command=self.master.destroy)
        self.quit_button.place(relx=0.9, rely=0.9)

        # packl buttons in scroll view
        counter = 0
        print(self.sounds)
        for sound in self.sounds:
            customtkinter.CTkButton(self.frame, text=sound["name"], command= lambda t=sound["id"]: self.play_sound(t)).pack(side=tk.BOTTOM, padx=10, pady=10)#, anchor="w")
            counter += 1
            
    def play_sound(self, t):
        r = requests.put(f'http://127.0.0.1:5000/play/{t}')
        
def main():
    root = tk.Tk()
    root.geometry("1280x720")
    customtkinter.set_appearance_mode("Dark")
    app = SoundboardApp(root)
    root.mainloop()

if __name__ == '__main__':
    main()