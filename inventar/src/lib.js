import { IncomingCategory, IncomingThing } from "./models";
import { server, colorStyles } from "./constants";

export async function createThing() {
  let formElement = document.querySelector("form");
  let formData = new FormData(formElement);

  let nt;
  let uploadedImage = await uploadImage();
  if (formData.get("category") === "new") {
    nt = new IncomingThing(
      formData.get("thing_name"),
      sessionStorage.getItem("loggedIn"),
      await createCategory(formData),
      formData.get("thing_description"),
      uploadedImage
    );
  } else {
    nt = new IncomingThing(
      formData.get("thing_name"),
      sessionStorage.getItem("loggedIn"),
      parseInt(formData.get("category")),
      formData.get("thing_description"),
      uploadedImage
    );
  }

  let res = await fetch(server + `/things/`, {
    method: "POST",
    body: JSON.stringify(nt),
  });

  if (res.ok) {
    return await readThings();
  } else {
    throw new Error(res.statusText);
  }
}

export async function createCategory(formData) {
  let nc = new IncomingCategory(
    formData.get("category_name"),
    formData.get("category_color"),
    sessionStorage.getItem("loggedIn")
  );

  let res = await fetch(server + `/categories/`, {
    method: "POST",
    body: JSON.stringify(nc),
  });

  if (res.ok) {
    let json = await res.json();
    return json.category_id;
  } else {
    throw new Error(res.statusText);
  }
}

export async function readThings() {
  let res = await fetch(
    server + `/things/${sessionStorage.getItem("loggedIn")}`,
    {
      method: "GET",
    }
  );

  if (res.ok) {
    let json = await res.json();
    return [...json].reverse();
  } else {
    throw new Error(res.statusText);
  }
}

export async function deleteThing(id) {
  let res = await fetch(server + `/things/${id}`, {
    method: "DELETE",
  });

  if (res.ok) {
    return await readThings();
  } else {
    throw new Error(res.statusText);
  }
}

export async function login(lu) {
  let res = await fetch(server + `/users/login`, {
    method: "POST",
    body: JSON.stringify(lu),
  });

  if (res.ok) {
    sessionStorage.setItem("loggedIn", lu.user_name);
    return true;
  } else {
    sessionStorage.setItem("loggedIn", "false");
    throw new Error(res.statusText);
  }
}

export function logout() {
  sessionStorage.setItem("loggedIn", "false");
  return false;
}

function validateEmail(email) {
  const re =
    /^(([^<>()[\]\\.,;:\s@\"]+(\.[^<>()[\]\\.,;:\s@\"]+)*)|(\".+\"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
  return re.test(email);
}

function validatePassword(password) {
  const re =
    /^(?=.*\d)(?=.*[a-z])(?=.*[A-Z])(?=.*[^a-zA-Z0-9])(?!.*\s).{8,15}$/;
  return re.test(password);
}

function validateUsername(username) {
  const re = /^[a-zA-Z][a-zA-Z0-9.$!@\-_]{2,}$/;
  return re.test(username);
}

export async function register(ru) {
  if (!validateEmail(ru.user_email)) throw new Error("Invalid e-mail");
  if (!validatePassword(ru.user_pwd)) throw new Error("Invalid password");
  if (!validateUsername(ru.user_name)) throw new Error("Invalid username");

  let res = await fetch(server + `/users/register`, {
    method: "POST",
    body: JSON.stringify(ru),
  });

  if (res.status === 201) {
    return true;
  } else if (res.status === 202) {
    throw new Error("existing username or email");
  } else {
    throw new Error(res.statusText);
  }
}

export async function readCategories() {
  let res = await fetch(
    server + `/categories/${sessionStorage.getItem("loggedIn")}`,
    {
      method: "GET",
    }
  );

  if (res.ok) {
    let json = await res.json();
    return [...json];
  } else {
    throw new Error(res.statusText);
  }
}

function pickRandomColorStyle() {
  return colorStyles[Math.floor(Math.random() * colorStyles.length)]
    .color_value;
}

async function uploadImage() {
  let thing_img = document.querySelector('input[type=file]').files[0];
  if (thing_img) {
    let form = new FormData();
    form.append("image", thing_img);
  
    let img_obj = await fetch("https://api.imgur.com/3/image/", {
      method: "POST",
      headers: {
        Authorization: "Client-ID daf84988891e873",
      },
      body: form,
    })
      .then((data) => data.json())

    return img_obj.data.link;
  } else {
    return "";
  }
}
