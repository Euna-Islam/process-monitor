function changeSorting(select) {
    console.log("sorting");
    let selectedOption = select.value;
    window.location.href = `?sort=${selectedOption}`;
}
