export type Spot = { lat: number; lng: number }

// The daily QR only works near where it's shown (see check_in in the API), so both the
// admin showing it and the student scanning it send their position.
// Like the camera, the browser only offers it over HTTPS or on localhost.
export function getPosition(): Promise<Spot> {
  return new Promise((resolve, reject) => {
    if (!navigator.geolocation) {
      return reject(new Error('This browser can not share its location, open the app over HTTPS'))
    }
    navigator.geolocation.getCurrentPosition(
      ({ coords }) => resolve({ lat: coords.latitude, lng: coords.longitude }),
      (error) =>
        reject(
          new Error(
            error.code === error.PERMISSION_DENIED
              ? 'Location access was denied, allow it in the browser site settings'
              : "Couldn't get your location, try again near a window or with GPS on"
          )
        ),
      { enableHighAccuracy: true, timeout: 15000, maximumAge: 60000 }
    )
  })
}
