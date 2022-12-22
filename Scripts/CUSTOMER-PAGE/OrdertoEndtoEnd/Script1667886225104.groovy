import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://apq-fe.ezrx.id/')

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/a_MASUK'))

WebUI.setText(findTestObject('Object Repository/Page_APL Customer Portal/input_APL CUSTOMER PORTAL_username'), '116223')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_APL Customer Portal/input_APL CUSTOMER PORTAL_password'), 
    'Q6+Gm1W9wsVP07t47TqoIA==')

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/label_APL CUSTOMER PORTAL_control-label'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/button_MASUK'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/p_Buat Pesanan'))

WebUI.setText(findTestObject('Object Repository/Page_APL Customer Portal/input_Bantuan_keywordSearchText'), 'danahbl')

WebUI.sendKeys(findTestObject('Object Repository/Page_APL Customer Portal/input_Bantuan_keywordSearchText'), Keys.chord(
        Keys.ENTER))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/button_Potensial program yang dapat digunak_d5d38c'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/button_Potensial program yang dapat digunak_d5d38c'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/button_Tambah Ke Keranjang'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/span_'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/i_Potensial program yang dapat digunakan_fa_134373'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/button_Tambah Ke Keranjang'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/span_'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/i_Lihat Semua Notifikasi_la la-shopping-cart'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/i_Lihat Semua Notifikasi_la la-shopping-cart'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/a_Lihat Semua'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/i_ARCH PROBE CONDITIONING_fa fa-minus'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/button_UPDATE KERANJANG'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/button_CHECKOUT'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/button_OK'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/button_CHECKOUT'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/button_OK'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/i_ARCH PROBE CONDITIONING_fa fa-minus'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/i_ARCH PROBE CONDITIONING_fa fa-minus'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/label_ARCH PROBE CONDITIONINGPT. ABBOTT PRO_4fea2a'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/button_Hapus'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/button_YA'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/div_BerhasilProduk berhasil dihapus dari keranjang'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/div_RINGKASANSub-total (8 Item) Rp. 104.370_8b0601'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/button_UPDATE KERANJANG'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/button_OK'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/p_Lihat Detail Harga'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/div_OK'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/button_OK'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/button_CHECKOUT'))

WebUI.closeBrowser()

