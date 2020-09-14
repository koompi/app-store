import { decode } from "jsonwebtoken";

export const USER = "USER";
export const DEVELOPER = "DEVELOPER";
export const ADMIN = "ADMIN";
export const GUEST = "GUEST";

// const UpdateToken = async (data) => {
// 	window.localStorage.setItem("token", JSON.stringify(data));
// };

export const auth_reducer = (state, action) => {
  if (action.target === "auth") {
    switch (action.type) {
      case "signin":
        let token = window.localStorage.getItem("token");
        if (token) {
          let data = decode(token);
          console.log(data);
          switch (data.role) {
            case "USER":
              return data;
            case "DEVELOPER":
              return data;
            case "ADMIN":
              return data;
            default:
              return { email: "", role: GUEST };
          }
        }
        break;
      case "signout":
        window.localStorage.removeItem("token");
        return { email: "", role: GUEST };
      default:
        break;
    }
  }
};
