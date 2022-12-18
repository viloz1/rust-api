import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { environment } from 'src/environments/environment';

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
  url = environment.apiUrl;

  login(email: String, password: String) {
    let header: HttpHeaders = this.header;

    header = header.set("Content-Type", "application/x-www-form-urlencoded");

    const r = this.http.post(this.url+"/api/auth/login",JSON.stringify({"email": email, "password": password}), {headers: header});
    return r;
  }

  logout() {
    let header: HttpHeaders = this.header;

    header = header.set("Content-Type", "application/x-www-form-urlencoded"
    );
    const r = this.http.post(this.url+"/api/auth/logout", {headers: header});
    return r;
  }

  check_login() {
    let header: HttpHeaders = this.header;
    header = header.set("Access-Control-Allow-Origin", "*");
    header = header.set("Content-Type", "application/x-www-form-urlencoded");

    const r = this.http.post(this.url+"/api/auth/check_login", {headers: header});
    return r;
  }


}
