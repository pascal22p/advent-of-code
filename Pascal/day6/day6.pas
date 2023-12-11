program Day6;
type
  DataType = record
    time : Int64;
    distance : Int64;
  end;

type
  RootsType = record
    root1 : Int64;
    root2 : Int64;
  end;

type
  DataListType = array [0..4] of DataType;

var testDataList: DataListType;
var dataList: DataListType;
var roots: RootsType;
var total: int64;
var i: integer;

function ceil(number: double): Int64;
begin
  if double(Trunc(number)) = number then
    ceil := Trunc(number)
  else
    ceil := Trunc(number) + 1;
end;

function get_roots(time: Int64; distance: Int64): RootsType;
var timef: double;
var distancef: double;
var root1, root2: double;

begin
  timef := double(time);
  distancef := double(distance + 1);

  root1 := (timef - Sqrt(timef * timef - 4.0 * distancef))/2.0;
  root2 := (timef + Sqrt(timef * timef - 4.0 * distancef))/2.0;

  if root1 < root2 then
    begin
       get_roots.root1 := Ceil(root1);
       get_roots.root2 := Trunc(root2)
     end
  else
    begin
       get_roots.root1 := Ceil(root2);
       get_roots.root2 := Trunc(root1)
     end;
end;

begin

  testDataList[0].time := 7;
  testDataList[0].distance := 9;
  testDataList[1].time := 15;
  testDataList[1].distance := 40;
  testDataList[2].time := 30;
  testDataList[2].distance := 200;

  dataList[0].time := 53;
  dataList[0].distance := 250;
  dataList[1].time := 91;
  dataList[1].distance := 1330;
  dataList[2].time := 67;
  dataList[2].distance := 1081;
  dataList[3].time := 68;
  dataList[3].distance := 1025;

  total := 1;
  for i:= 0 to 2 do
    begin
      roots := get_roots(testDataList[i].time, testDataList[i].distance);
      total := total * (roots.root2 - roots.root1 + 1);
      (* writeln('Intermediate ', roots.root1, ' ', roots.root2, ' ', roots.root2 - roots.root1 + 1) *)
    end;

  writeln ('Test part 1: ', total);


  total := 1;
  for i:= 0 to 3 do
    begin
      roots := get_roots(dataList[i].time, dataList[i].distance);
      total := total * (roots.root2 - roots.root1 + 1);
      (* writeln('Intermediate ', roots.root1, ' ', roots.root2, ' ', roots.root2 - roots.root1 + 1) *)
    end;

  writeln ('Part 1: ', total);
end.

