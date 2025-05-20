from graphviz import Digraph

fsm = Digraph('Finite State Machine', format = 'png')
fsm.attr(rankdir='D', nodesep='4.0', ranksep='2.0')

# states
fsm.node("S0", "Idle")
fsm.node("S1", "CheckServerStatus")
fsm.node("S2", "SendInitConnKey")
fsm.node("S3", "ConnectionTerminated")
fsm.node("S4", "Heartbeat")
fsm.node("S5", "GotCommandFromServer")
fsm.node("S6", "SendResponseToServer")



# Transisions
fsm.edge("S0", "S1", label = "Client Run")
fsm.edge("S1", "S2", label = "SERVER_UP")
fsm.edge("S1", "S3", label = "SERVER_DOWN")
fsm.edge("S2", "S4", label = "KEY_EXCHANGE_SUCCEEDED")
fsm.edge("S4", "S4", label = "HB_NO_ACTION")
fsm.edge("S3", "S4", label = "HB_CONNECTION_TERMINATED")
fsm.edge("S4", "S5", label = "COMMAND_MSG")
fsm.edge("S5", "S6", label = "COMMAND_GET_SUCCEEDED")
fsm.edge("S5", "S5", label = "COMMAND_GET_FAILED")
fsm.edge("S6", "S6", label = "SEND_RESPONSE_FAILED")
fsm.edge("S6", "S4", label = "SEND_RESPONSE_SUCCEEDED", tailport='w', headport='w')

# Save / Render
fsm.render('fsm_diagram', view=True)