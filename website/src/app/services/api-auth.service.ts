import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { environment } from 'src/environments/environment';

@Injectable({
  providedIn: 'root'
})
export class ApiAuthService {

  constructor(private http: HttpClient) {

   }


  header = new HttpHeaders();
  url = environment.apiUrl;

  login(email: String, password: String) {
    let header: HttpHeaders = this.header;

    header = header.set("Content-Type", "application/x-www-form-urlencoded");
    

    const r = this.http.post(this.url+"/api/auth/login",JSON.stringify({"email": email, "password": password}), {headers: header, withCredentials: true});
    return r;
  }

  logout() {
    let header: HttpHeaders = this.header;

    header = header.set("Content-Type", "application/x-www-form-urlencoded");

    const r = this.http.post(this.url+"/api/auth/logout", null, {headers: header, withCredentials: true});
    return r;
  }

  check_login() {
    let header: HttpHeaders = this.header;

    const r = this.http.post(this.url+"/api/auth/check_login", null, {headers: header, withCredentials: true});
    return r;
  }


}
