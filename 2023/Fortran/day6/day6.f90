program day6
use iso_fortran_env, only: int64, real64
implicit none

type runType
    integer(int64) :: time
    integer(int64) :: distance
end type

type(runType), dimension(3) :: testDataList
type(runType), dimension(4) :: dataList

integer(int64) :: total
integer(int64), dimension(2) :: roots
integer i


testDataList(1) = runType(7, 9)
testDataList(2) = runType(15, 40)
testDataList(3) = runType(30, 200)

dataList(1) = runType(53, 250)
dataList(2) = runType(91, 1330)
dataList(3) = runType(67, 1081)
dataList(4) = runType(68, 1025)

total = 1
do i = 1, 3
    roots = get_roots(testDataList(i)%time, testDataList(i)%distance);
    total = total * (roots(2) - roots(1) + 1);
    ! writeln('Intermediate ', roots.root1, ' ', roots.root2, ' ', roots.root2 - roots.root1 + 1) *)
end do

write(*,*) 'Test part 1: ', total

total = 1
do i = 1, 4
    roots = get_roots(dataList(i)%time, dataList(i)%distance);
    total = total * (roots(2) - roots(1) + 1);
    ! writeln('Intermediate ', roots.root1, ' ', roots.root2, ' ', roots.root2 - roots.root1 + 1) *)
end do

write(*,*) 'Part 1: ', total

contains 

function get_roots(time, distance) result(roots)
    implicit none
    integer(int64) :: time, distance
    real(real64) :: timef, distancef, root1, root2
    integer(int64), dimension(2) :: roots

    timef = time
    distancef = distance + 1
    
    root1 = (timef - Sqrt(timef * timef - 4.0_real64 * distancef))/2.0_real64;
    root2 = (timef + Sqrt(timef * timef - 4.0_real64 * distancef))/2.0_real64;

    if (root1 < root2) then
        roots(1) = ceiling(root1);
        roots(2) = floor(root2)
    else        
        roots(1) = ceiling(root2);
        roots(2) = floor(root1)
    end if
end function


end
