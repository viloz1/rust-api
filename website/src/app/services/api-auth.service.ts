import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class ApiAuthService {

  constructor(private http: HttpClient) {
    this.header.set("Access-Control-Allow-Origin", "*")
    this.header.set("Access-Control-Allow-Methods", "DELETE, POST, GET, OPTIONS")
    this.header.set("Access-Control-Allow-Headers", "Content-Type, Authorization, X-Requested-With")
   }

  header = new HttpHeaders();

  login(email: String, password: String) {
    let header: HttpHeaders = this.header;

    header = header.set("Content-Type", "application/x-www-form-urlencoded");

    const r = this.http.post("http://localhost:4200/api/auth/login",JSON.stringify({"email": email, "password": password}), {headers: header});
    return r;
  }

  logout() {
    let header: HttpHeaders = this.header;

    header = header.set("Content-Type", "application/x-www-form-urlencoded"
    );
    const r = this.http.post("http://localhost:4200/api/auth/logout", {headers: header});
    return r;
  }

  check_login() {
    let header: HttpHeaders = new HttpHeaders();

    header = header.set("Content-Type", "application/x-www-form-urlencoded");
    const r = this.http.post("http://localhost:4200/api/auth/check_login", {headers: header});
    return r;
  }


}
