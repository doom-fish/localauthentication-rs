import Foundation
import LocalAuthentication

@_cdecl("la_error_copy_domain")
public func la_error_copy_domain(
    _ outDomain: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let outDomain else {
        return laFail(LABridgeError.invalidArgument("missing output pointer for LAErrorDomain"), errorOut)
    }

    outDomain.pointee = laCString(LAErrorDomain)
    return LA_OK
}
