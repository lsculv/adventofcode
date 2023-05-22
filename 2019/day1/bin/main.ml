open Core

let rec sum l =
  match l with
  | [] -> 0
  | hd :: tl -> hd + sum tl
;;

let rec map f = function
  | [] -> []
  | hd :: tl -> f hd :: map f tl
;;

let part_1 input =
  let fuel_from_mass fuel = (int_of_string fuel / 3) - 2 in
  input |> String.split_lines |> map fuel_from_mass |> sum
;;

let part_2 input =
  let rec total_fuel n =
    let x = (n / 3) - 2 in
    match x > 0 with
    | false -> 0
    | true -> x + total_fuel x
  in
  let total_fuel_from_mass fuel = total_fuel (int_of_string fuel) in
  input |> String.split_lines |> map total_fuel_from_mass |> sum
;;

let input = In_channel.read_all "../inputs/day1.txt"

let () =
  printf "Part 1: %d\n" (part_1 input);
  printf "Part 2: %d\n" (part_2 input)
;;
