#Link to problem: https://adventofcode.com/2017/day/25

import re

NUMBER_STATE_DEFINITION_LINES = 9

src = "../res/d25"
states_file = open(src)
input_lines = states_file.read().split('\n')
states_file.close()

# PART 1
re_begin_state = "Begin in state (.)."
re_check_sum_steps = "Perform a diagnostic checksum after (.*) steps."
re_state = "In state (.):"
re_write_value = "Write the value (.)."
re_move_direction = "Move one slot to the (.*)."
re_next_state = "Continue with state (.)."

input_lines = [line for line in input_lines if line != ""]
cur_state = re.findall(re_begin_state, input_lines[0])[0]
check_sum_steps = int(re.findall(re_check_sum_steps, input_lines[1])[0])
input_lines = input_lines[2:]

cursor = 0
tape = {}   # cursor to value, default is 0
states = {} # state to (0: (write value, movement delta, next state), 1: (write value, movement delta, next state))

while len(input_lines) > 0:
  cur_state_lines = input_lines[:NUMBER_STATE_DEFINITION_LINES]
  state = re.findall(re_state, cur_state_lines[0])[0]
  
  write_val = int(re.findall(re_write_value, cur_state_lines[2])[0])
  move_delta = 1 if re.findall(re_move_direction, cur_state_lines[3])[0] == "right" else -1
  next_state = re.findall(re_next_state, cur_state_lines[4])[0]
  case_zero = (write_val, move_delta, next_state)

  write_val = int(re.findall(re_write_value, cur_state_lines[6])[0])
  move_delta = 1 if re.findall(re_move_direction, cur_state_lines[7])[0] == "right" else -1
  next_state = re.findall(re_next_state, cur_state_lines[8])[0]
  case_one = (write_val, move_delta, next_state)

  states[state] = (case_zero, case_one)
  input_lines = input_lines[NUMBER_STATE_DEFINITION_LINES:]

for _ in range(check_sum_steps):
  current = tape[cursor] if cursor in tape else 0
  (write_val, move_delta, next_state) = states[cur_state][current]
  tape[cursor] = write_val
  cursor += move_delta
  cur_state = next_state

print(f'(Part1) Diagnostic checksum: {sum([tape[k] for k in tape])}')