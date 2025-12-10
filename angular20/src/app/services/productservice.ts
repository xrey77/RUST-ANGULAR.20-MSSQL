import { Injectable, inject } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import {  Observable } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class Productservice {

  private http = inject(HttpClient);

  public sendSearchRequest(page: any, keyword: any): Observable<any>
  {
    const options = {
      headers: new HttpHeaders({
        'Content-Type':  'application/json',
      })
    };
    return this.http.get<any>(`http://127.0.0.1:3000/auth/products/search/${page}/${keyword}`, options);
  }

  public sendProductRequest(page: any): Observable<any>
  {
    const options = {
      headers: new HttpHeaders({
        'Content-Type':  'application/json'
        // 'Authorization': 'jwt-token'
      })
    };
    return this.http.get<any>(`http://127.0.0.1:3000/auth/products/list/${page}`, options);

  }  
}
