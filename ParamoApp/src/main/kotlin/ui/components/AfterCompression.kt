import androidx.compose.foundation.layout.*
import androidx.compose.foundation.text.ClickableText
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.AnnotatedString
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.capitalize
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.sp

@Composable
fun AfterCompression(width: Dp){
    var deleteOriginal by remember { mutableStateOf(false) }
    var openLocation by remember { mutableStateOf(false) }

    Row(
        modifier = Modifier.width(width),
        horizontalArrangement = Arrangement.SpaceBetween
    ) {
        Row(
            verticalAlignment = Alignment.CenterVertically
        ) {
            Checkbox(
                checked = deleteOriginal,
                onCheckedChange = { deleteOriginal = it }
            )

            ClickableText(
                text = AnnotatedString("Delete Original File"),
                style = TextStyle(
                    fontSize = 16.sp,
                    color = Color(0x99000000)
                ),
                onClick = {
                    deleteOriginal = !deleteOriginal
                }
            )
        }

        Row(
            verticalAlignment = Alignment.CenterVertically
        ) {
            Checkbox(
                checked = openLocation,
                onCheckedChange = { openLocation = it }
            )

            ClickableText(
                text = AnnotatedString("Open Compressed File Location"),
                style = TextStyle(
                    fontSize = 16.sp,
                    color = Color(0x99000000)
                ),
                onClick = {
                    openLocation = !openLocation
                }
            )
        }
    }
}