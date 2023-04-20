const yearsMap = {
    cat: [15, 9, 4],
    dog: [15, 9, 5]
}
export function humanYearsCatYearsDogYears(humanYears: number): [number, number, number] {
    let catYears = 0;
    let dogYears = 0;
    for (let i = 0; i < humanYears; i++) {
        catYears += yearsMap.cat[Math.min(2, i)];
        dogYears += yearsMap.dog[Math.min(2, i)];
       
    }
    return [humanYears, catYears, dogYears];
}

console.log(humanYearsCatYearsDogYears(1));
console.log(humanYearsCatYearsDogYears(2));
console.log(humanYearsCatYearsDogYears(10));