export interface windowType {
    title?:string,
    decorations?:boolean,
    transparent?:boolean,
    skipTaskbar?:boolean,
    x?:number,
    y?:number,
    width:number,
    height:number,
    maxHeight?:number,
    maxWidth?:number,
    url:string,
    alwaysOnTop?:boolean,
    fileDropEnabled?:boolean,
    center?:boolean
}

export interface lnk {
    title?:string,
    src:string,
    icon:string,
    type?:"tauricommand"|"exe"|"fn"
}

export interface editorType {
    color:string,
    opacity:number,
    value:string,
    date?:string,
}

export interface location {
    name:string // 地区/城市名称
    id:string // 地区/城市ID
    lat:string // 地区/城市纬度
    lon:string // 地区/城市经度
    adm2:string // 地区/城市的上级行政区划名称
    adm1:string // 地区/城市所属一级行政区域
    country:string // 地区/城市所属国家名称
    tz:string //  地区/城市所在时区
    utcOffset:string // 地区/城市目前与UTC时间偏移的小时数，参考详细说明
    isDst:string // 地区/城市是否当前处于夏令时。1 表示当前处于夏令时，0 表示当前不是夏令时。
    type:string // 地区/城市的属性
    rank:string // 地区评分
    fxLink:string // 该地区的天气预报网页链接，便于嵌入你的网站或应用
}

export interface nowWeather {
    obsTime:string, //数据观测时间
    temp:string, //温度
    feelsLike:string, //体感温度
    icon:string, // 图标
    text:string, // 天气文字描述
    wind360:string, // 风向360角度
    windDir:string, //风向
    windScale:string, // 风力等级
    windSpeed:string, //风速 公里/小时
    humidity:string, //相对湿度
    precip:string, //当前小时累计降水量
    pressure:string, //大气压强
    vis:string, //能见度
    cloud?:string, //云量
    dew?:string //露点温度
}


//wallpaper

export interface wallpaperType {
    id: string,
    url: string,
    short_url: string,
    uploader?: {
      username: string,
      group: string,
      avatar: {
        "200px": string,
        "128px": string,
        "32px": string,
        "20px": string
      }
    },
    views?: number,
    favorites?: number,
    source?: string,
    purity?: string,
    category?: string,
    dimension_x?: number,
    dimension_y?: number,
    resolution?: string,
    ratio?: string,
    file_size?: string,
    file_type?: string,
    created_at?: string,
    colors?: string[],
    path?: string,
    thumbs?: {
      large: string,
      original: string,
      small: string
    },
    tags?: [
      {
        id: number,
        name: string,
        alias: string,
        category_id: number,
        category: string,
        purity: string,
        created_at: string
      }
    ]
  }