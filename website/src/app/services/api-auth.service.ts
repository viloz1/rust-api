import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class ApiAuthService {

  constructor(private http: HttpClient) { }

  login(email: String, password: String) {
    let header: HttpHeaders = new HttpHeaders();

    header = header.set("Content-Type", "application/x-www-form-urlencoded");

    const r = this.http.post("http://localhost:4200/api/auth/login",JSON.stringify({"email": email, "password": password}), {headers: header});
    return r;
  }

  logout() {
    let header: HttpHeaders = new HttpHeaders();

    header = header.set("Content-Type", "application/x-www-form-urlencoded");
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
