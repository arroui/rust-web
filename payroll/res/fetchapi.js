function getInputValue(){
    var req = document.getElementById("pollapi").value;

    fetch(req)
        .then(response => {
            document.getElementById("response").innerHTML = response.status + ' ' + response.statusText;
            response.text()
                .then(result => document.getElementById("result").innerHTML = result )
                .catch(error => alert(error));
        })
        .catch(error => alert(error))
        .finally(() => document.getElementById("pollapi").value = '');
}