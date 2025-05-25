import os

# Var to hold the script name used mainly in help message
SCRIPT_NAME = os.path.basename(__file__)

# This list holds all the avaliable execution commands if any new command is added it must be updated
<<<<<<< HEAD
AVALIABLE_EXECUTION_COMMANDS = ['-h', '--help', '-l', '--logo', '-c', '--commands']
=======
AVALIABLE_EXECUTION_COMMANDS = ['list','select','help','exit','logo']
>>>>>>> 83d37bc438b82b2eb238380e0c059d936a5d7653

# The logo as raw string
LOGO_STR = r"""
                                                         _..._                   
                                                      .-'_..._''.     .-''-.     
   _..._                _..._                       .' .'      '.\  .' .-.  )    
 .'     '.            .'     '.                    / .'            / .'  / /     
.   .-.   .          .   .-.   .          .-,.--. . '             (_/   / /      
|  '   '  |    __    |  '   '  |    __    |  .-. || |                  / /       
|  |   |  | .:--.'.  |  |   |  | .:--.'.  | |  | || |                 / /        
|  |   |  |/ |   \ | |  |   |  |/ |   \ | | |  | |. '                . '         
|  |   |  |`" __ | | |  |   |  |`" __ | | | |  '-  \ '.          .  / /    _.-') 
|  |   |  | .'.''| | |  |   |  | .'.''| | | |       '. `._____.-'/.' '  _.'.-''  
|  |   |  |/ /   | |_|  |   |  |/ /   | |_| |         `-.______ //  /.-'_.'      
|  |   |  |\ \._,\ '/|  |   |  |\ \._,\ '/|_|                  `/    _.'         
'--'   '--' `--'  `" '--'   '--' `--'  `"                      ( _.-'            
"""

# Function: print the logo
def print_logo():
    print(LOGO_STR)

# Function: return the help message
def execution_args_help_message():
    
    help_msg = f"""\n
            -------------------------------------------------------------------------------------
           | NanarC2 Server is a very simple reverse shell handler combined with NanarC2 client. |
            -------------------------------------------------------------------------------------   
    
    {LOGO_STR}
<<<<<<< HEAD
    
    [+] Execute:
        python {SCRIPT_NAME}
    
    [+] Parameters:
        -h, --help          Print the help message
        -l, --logo          Print the logo
        -c, --commands      Print the commands the user can execute inside the running server
        
        
    [+] How to use:
        1) Run the server script
                python {SCRIPT_NAME}
        2) Generate the password using:
                --conn-init-pass, -cip [password_string]
        3) Add the generated hash to the client source code and compile it to release version
                cargo build --release
        4)  Execute the client on the target machine and wait for the connection --- ENJOY UWU ~/////~ ---
    
=======


>>>>>>> 83d37bc438b82b2eb238380e0c059d936a5d7653
    [+] Made By:
        1) MuhammadMuazen: https://github.com/MuhammadMuazen/ (Client coder and other stuff)
        2) YazanAlJedawi: https://github.com/YazanAlJedawi (Server coder)
        3) Hussien_Shanan: https://github.com/SHNN267 (Server coder)
<<<<<<< HEAD
"""                                                                                               
    return help_msg

# Function: return the avaliable commands after running the server
def avaliable_commands():

    # TODO add more commands if needed
    commands = """[+] Commands List:
    
    -cip, --conn-init-pass [string]     Generate the initlization password that will be used in the connection
                                        initlization between the server and the client 
    -l,   --listen                      Start listening for any incoming connections
"""
    return commands
=======
    
    
      
        
    [+] How to use:

    The server`s shell "turtle" is configured with a few commands that you can make use of:
    
    help:          prints this message.
    
    list:          lists all active connections to the server, each client with it`s ID.

    
    select:        supply it with the ID of the client of choice <<use list command to optain the ID>> and you shall have access
                   to the client terminal.
                   >select <ID>
            

    logo:          prints the logo.

    exit:          shutdown the server.


    [*] Note: you can use the select command to switch to another client while you are accessing one.
    
"""                                                                                               
    return help_msg

def interface():
    inter = f"""\n
            -------------------------------------------------------------------------------------
           | NanarC2 Server is a very simple reverse shell handler combined with NanarC2 client. |
            -------------------------------------------------------------------------------------   
    
    {LOGO_STR}


    [+] Made By:
        1) MuhammadMuazen: https://github.com/MuhammadMuazen/ (Client coder and other stuff)
        2) YazanAlJedawi: https://github.com/YazanAlJedawi (Server coder)
        3) Hussien_Shanan: https://github.com/SHNN267 (Server coder)
    
"""
    return inter




# Function: return the avaliable commands after running the server
# def avaliable_commands():

#     # TODO add more commands if needed
#     commands = """[+] Commands List:
    
#     -cip, --conn-init-pass [string]     Generate the initlization password that will be used in the connection
#                                         initlization between the server and the client 
#     -l,   --listen                      Start listening for any incoming connections
# """
#     return commands
>>>>>>> 83d37bc438b82b2eb238380e0c059d936a5d7653
