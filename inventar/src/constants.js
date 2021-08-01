export let server = window.location.hostname == "localhost" || window.location.hostname == "127.0.0.1" ? "http://localhost:8000" : "https://inventar-api.herokuapp.com";

export let colorStyles = [
    {color_name: "Blue", color_value:"text-white bg-primary"},
    {color_name: "Gray", color_value:"text-white bg-secondary"},
    {color_name: "Green", color_value:"text-white bg-success"},
    {color_name: "Red", color_value:"text-white bg-danger"},
    {color_name: "Yellow", color_value:"text-dark bg-warning"},
    {color_name: "Cyan", color_value:"text-dark bg-info"},
    {color_name: "White", color_value:"text-dark bg-light"},
    {color_name: "Black", color_value:"text-white bg-dark"}
]