import axios from 'axios'
// 9cda7ed49a914d5eb6987706d642da65
export const getpoi = function(city:string):any{
    return axios.get('https://geoapi.qweather.com/v2/city/lookup?location='+city+'&key=9cda7ed49a914d5eb6987706d642da65')
}

export const getWeather = function(location:string):any{
    return axios.get('https://devapi.qweather.com/v7/weather/now?location='+location+'&key=9cda7ed49a914d5eb6987706d642da65')
}

export const getwallpaper = function(){
    return axios.get('https://wallhaven.cc/api/v1/search?sorting=random');
}