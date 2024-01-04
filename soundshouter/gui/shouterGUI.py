import tkinter as tk
import customtkinter
#from pygame import mixer
import requests

class SoundboardApp:
    def __init__(self, master):
        self.master = master
        self.master.title("Soundshouter V13.37")
        self.api_url = ""

        # Create Frames
        self.frame_left = customtkinter.CTkScrollableFrame(master=self.master, width=915, height=600)
        self.frame_left.place(relx=0.01, rely=0.1, anchor="nw")

        self.frame_right = customtkinter.CTkFrame(master=self.master, width=300, height=612)
        self.frame_right.place(relx=0.75, rely=0.1, anchor="nw")

        #self.get_sounds()
        self.create_widgets()

    # def get_sounds(self):
    #     self.request_sounds()

    def request_sounds(self):
        r = requests.get(f'{self.api_url}/sounds')
        return r.json()

    def request_categories(self):
        r = requests.get(f'{self.api_url}/categories')
        return r.json()

    def request_subcategories(self):
        r = requests.get(f'{self.api_url}/subcategories')
        return r.json()

    def create_widgets(self):
        # cleanup
        for widget in self.frame_left.winfo_children():
            widget.destroy()
        try:
            self.combobox_cat.destroy()
        except:
            pass

        # Create Head label
        self.head_label = customtkinter.CTkLabel(self.master, text="Spielen du H...", fg_color="grey", width=100, height=50, text_color="blue", font=("Courier", 44))
        self.head_label.place(relx=0.01, rely=0.02, anchor="nw")

        # Create url_textfield
        customtkinter.CTkLabel(self.frame_right, text="API_URL:", fg_color="grey", text_color="blue", font=("Courier", 12)).place(relx=0.05, rely=0.04, anchor="nw")
        customtkinter.CTkLabel(self.frame_right, text="Status:", fg_color="grey", text_color="blue", font=("Courier", 12)).place(relx=0.05, rely=0.13, anchor="nw")

        self.r_status = customtkinter.CTkLabel(self.frame_right, text="NONE", fg_color="grey", text_color="blue", font=("Courier", 12), wraplength=200)
        self.r_status.place(relx=0.25, rely=0.13, anchor="nw")

        self.var = tk.StringVar()
        self.var.set("http://127.0.0.1:5000")

        self.api_url_text_field = customtkinter.CTkEntry(self.frame_right, textvariable = self.var, placeholder_text="API URL", width=250)
        self.api_url_text_field.place(relx=0.05, rely=0.08, anchor="nw")
        self.api_url_text_field.bind('<KeyRelease>', self.callback)
        self.api_url_text_field.focus_set()
        
        # Catergories comboboxes
        customtkinter.CTkLabel(self.frame_right, text="Catergorie:", fg_color="grey", text_color="blue", font=("Courier", 12)).place(relx=0.05, rely=0.5, anchor="nw")
        self.combobox_cat = customtkinter.CTkComboBox(master=self.frame_right,
                                     values=[],
                                     command=self.get_categorie,
                                     variable="categorie")
        self.combobox_cat.place(relx=0.05, rely=0.55, anchor="nw")

        customtkinter.CTkLabel(self.frame_right, text="Subcatergorie:", fg_color="grey", text_color="blue", font=("Courier", 12)).place(relx=0.05, rely=0.6, anchor="nw")
        self.combobox_subcat = customtkinter.CTkComboBox(master=self.frame_right,
                                     values=[],
                                     command=self.get_subcategorie,
                                     variable="categorie")
        self.combobox_subcat.place(relx=0.05, rely=0.65, anchor="nw")

        # Create quit button
        self.quit_button = customtkinter.CTkButton(self.frame_right, text="Beenden", command=self.master.destroy)
        self.quit_button.place(relx=0.5, rely=0.92, anchor="nw")

        # Create refresh button
        self.quit_button = customtkinter.CTkButton(self.frame_right, text="Refresh", command=self.create_widgets)
        self.quit_button.place(relx=0.02, rely=0.92, anchor="nw")

        self.callback("")

    def get_categorie(self, t):
        # Get id for selected categorie
        for cat in self.categories:
            if cat["name"] == t:
                return cat["id"]

    def get_subcategorie(self, t):
        # TODO CALLED WHJEN CAT IS CHOSEN USER HERE TO CALL BUILD BUTTONS
        # Get id for selected subcategorie
        for subcat in self.subcategories:
            if subcat["name"] == t:
                return subcat["id"]

    def build_buttons(self):
        #print(self.combobox_cat.get())
        # pack buttons in scroll view
        for sound in self.sounds:
            customtkinter.CTkButton(self.frame_left, text=f'{sound["name"]}({sound["play_count"]})', command= lambda t=sound["id"]: self.play_sound(t)).pack(side=tk.BOTTOM, padx=10, pady=2, fill="x")

    def extract_cat(self):
        cat_list = []
        for entry in self.categories:
            cat_list.append(entry["name"])
        
        return cat_list

    def callback(self, t):
        self.api_url = self.api_url_text_field.get()
        
        # Check Api status
        try: 
            r = requests.get(f'{self.api_url}/sounds')
            self.r_status.configure(text = str(r.status_code))

            # request infos from openapi
            self.sounds = self.request_sounds()
            self.categories = self.request_categories()
            self.subcategories = self.request_subcategories()

            # cleanup
            for widget in self.frame_left.winfo_children():
                widget.destroy() 

            # pack buttons in scroll view
            self.build_buttons()

            # fill catergorie anmd clean
            self.combobox_cat.configure(values=[])
            self.combobox_cat.configure(values=self.extract_cat())
        except Exception as e:
            # cleanup
            for widget in self.frame_left.winfo_children():
                widget.destroy()
            self.r_status.configure(text = str(e))
            self.combobox_cat.configure(values=[])
            
    def play_sound(self, t):
        r = requests.put(f'{self.api_url}/play/{t}')
        
def main():
    root = tk.Tk()
    root.geometry("1280x720")
    customtkinter.set_appearance_mode("Dark")
    app = SoundboardApp(root)
    root.mainloop()

if __name__ == '__main__':
    main()