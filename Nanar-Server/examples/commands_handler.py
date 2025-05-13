import os

# Var to hold the script name used mainly in help message
SCRIPT_NAME = os.path.basename(__file__)

# This list holds all the avaliable execution commands if any new command is added it must be updated
AVALIABLE_EXECUTION_COMMANDS = ['-h', '--help', '-l', '--logo', '-c', '--commands']

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
    
    [+] Made By:
        1) MuhammadMuazen: https://github.com/MuhammadMuazen/ (Client coder and other stuff)
        2) YazanAlJedawi: https://github.com/YazanAlJedawi (Server coder)
        3) Hussien_Shanan: https://github.com/SHNN267 (Server coder)
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
